#![allow(dead_code)]
#![allow(unused_imports)]

use structopt::StructOpt;
use rand::{self, RngCore};

use std::ops::RangeBounds;
use std::ops::Bound;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;
use std::collections::{BTreeSet, BTreeMap, HashMap, hash_map, btree_map};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io;

mod constraints;
use constraints::*;


//// prng stuff ////

#[derive(Debug, Clone, PartialEq, Eq)]
struct Xorshift64(u64);

impl Xorshift64 {
    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn next(&mut self) -> f64 {
        (self.next_u64() as f64) / (2.0f64.powf(64.0))
    }

    fn bool(&mut self, p: f64) -> bool {
        self.next() < p
    }

    fn poisson(&mut self, p: f64) -> usize {
        let mut count = 0;
        while self.bool(p) {
            count += 1;
        }
        count
    }

    fn range<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
        let start = match range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x+1,
            Bound::Unbounded => 0,
        };
        let stop = match range.end_bound() {
            Bound::Included(&x) => x+1,
            Bound::Excluded(&x) => x,
            Bound::Unbounded => unreachable!(),
        };

        (self.next() * ((stop-start) as f64)) as usize + start
    }
}



//// generate bubbles ////

#[derive(Debug, Clone)]
struct Bubble {
    x: isize,
    y: isize,
    r: usize,
    parent: Option<Rc<RefCell<Bubble>>>,
}

fn sq(a: usize) -> f64 {
    a as f64 * a as f64
}

fn distsq(a: (isize, isize), b: (isize, isize)) -> f64 {
    let d_x = (b.0 as f64) - (a.0 as f64);
    let d_y = (b.1 as f64) - (a.1 as f64);
    d_x*d_x + d_y*d_y
}

fn dist(a: (isize, isize), b: (isize, isize)) -> f64 {
    distsq(a, b).sqrt()
}

fn gen_bubbles(
    prng: &mut Xorshift64,
    size: usize,
    bubble_p: f64,
    hallway_p: f64,
    smallest: usize,
    clearance: usize,
) -> (usize, usize, Vec<Rc<RefCell<Bubble>>>) {
    // initialize our first bubble with a random size
    let mut bubbles = vec![
        Rc::new(RefCell::new(Bubble{
            x: 0,
            y: 0,
            r: 1 + prng.poisson(bubble_p),
            parent: None
        }))
    ];
    let mut used = bubbles[0].borrow().r;

    while used < size {
        // choose a bubble
        let parent = &bubbles[prng.range(0..bubbles.len())];
        // choose a direction
        let (dir_x, dir_y) = match prng.range(0..4) {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => unreachable!(),
        };
        // choose a size
        let r = smallest + prng.poisson(bubble_p);

        // calculate new position
        let hallway = clearance + prng.poisson(hallway_p);
        let x = parent.borrow().x
            + dir_x*((parent.borrow().r + r + hallway) as isize);
        let y = parent.borrow().y
            + dir_y*((parent.borrow().r + r + hallway) as isize);

        // but wait, is there a collision?
        let mut collision = false;
        for bubble in &bubbles {
            if Rc::ptr_eq(&bubble, parent) {
                continue;
            }

            // check bubble collision
            if
                distsq((x, y), (bubble.borrow().x, bubble.borrow().y))
                    <= sq(r + bubble.borrow().r + clearance)
            {
                collision = true;
                break;
            }
        }
        if collision {
            continue;
        }

        // no? ok add to our bubbles
        let bubble = Rc::new(RefCell::new(Bubble{
            x: x,
            y: y,
            r: r,
            parent: Some(parent.clone()),
        }));
        bubbles.push(bubble);
        used += r;
    }

    // find bounds
    let (mut lower_x, mut lower_y) = (0, 0);
    let (mut upper_x, mut upper_y) = (1, 1);
    for bubble in &bubbles {
        lower_x = cmp::min(
            lower_x, bubble.borrow().x-bubble.borrow().r as isize
        );
        lower_y = cmp::min(
            lower_y, bubble.borrow().y-bubble.borrow().r as isize
        );
        upper_x = cmp::max(
            upper_x, bubble.borrow().x+bubble.borrow().r as isize
        );
        upper_y = cmp::max(
            upper_y, bubble.borrow().y+bubble.borrow().r as isize
        );
    }
    let width = (upper_x+1 - lower_x) as usize;
    let height = (upper_y+1 - lower_y) as usize;

    // shift all bubbles into the range 0,0 => width,height
    for bubble in &bubbles {
        bubble.borrow_mut().x -= lower_x;
        bubble.borrow_mut().y -= lower_y;
    }

    (width, height, bubbles)
}

// render small map
fn render_small_map(
    width: usize,
    height: usize,
    bubbles: &[Rc<RefCell<Bubble>>],
    swidth: usize,
    sheight: usize,
) -> (usize, usize, Vec<u8>) {
    let mut smap = vec![b' '; swidth*sheight];
    let scale_x = swidth as f64 / width as f64;
    let scale_y = sheight as f64 / height as f64;

    // show hallways
    for bubble in bubbles {
        let x = (bubble.borrow().x as f64 * scale_x) as usize;
        let y = (bubble.borrow().y as f64 * scale_y) as usize;
        if let Some(parent) = &bubble.borrow().parent {
            let p_x = (parent.borrow().x as f64 * scale_x) as usize;
            let p_y = (parent.borrow().y as f64 * scale_y) as usize;
            for x_ in cmp::min(x, p_x) ..= cmp::max(x, p_x) {
                if smap[x_+y*swidth] == b'|' {
                    smap[x_+y*swidth] = b'+';
                } else {
                    smap[x_+y*swidth] = b'-';
                }
            }
            for y_ in cmp::min(y, p_y) ..= cmp::max(y, p_y) {
                if smap[x+y_*swidth] == b'-' {
                    smap[x+y_*swidth] = b'+';
                } else {
                    smap[x+y_*swidth] = b'|';
                }
            }
        }
    }

    // show bubbles
    for bubble in bubbles {
        let x = (bubble.borrow().x as f64 * scale_x) as usize;
        let y = (bubble.borrow().y as f64 * scale_y) as usize;
        smap[x+y*swidth] = b'o';
    }

    (swidth, sheight, smap)
}

// render bubble map
fn render_bubble_map(
    width: usize,
    height: usize,
    bubbles: &[Rc<RefCell<Bubble>>],
) -> (usize, usize, Vec<u8>) {
    let mut bmap = vec![b' '; width*height];

    // show bubbles
    for bubble in bubbles {
        let x = bubble.borrow().x as usize;
        let y = bubble.borrow().y as usize;
        let r = bubble.borrow().r;
        for y_ in 0..height {
            for x_ in 0..width {
                if
                    distsq((x_ as isize, y_ as isize), (x as isize, y as isize))
                        <= sq(r)
                {
                    bmap[x_+y_*width] = b'.';
                }
            }
        }
    }

    // show hallways
    for bubble in bubbles {
        let x = bubble.borrow().x as usize;
        let y = bubble.borrow().y as usize;
        if let Some(parent) = &bubble.borrow().parent {
            let p_x = parent.borrow().x as usize;
            let p_y = parent.borrow().y as usize;
            for x_ in cmp::min(x, p_x) ..= cmp::max(x, p_x) {
                if bmap[x_+y*width] == b'|' {
                    bmap[x_+y*width] = b'+';
                } else {
                    bmap[x_+y*width] = b'-';
                }
            }
            for y_ in cmp::min(y, p_y) ..= cmp::max(y, p_y) {
                if bmap[x+y_*width] == b'-' {
                    bmap[x+y_*width] = b'+';
                } else {
                    bmap[x+y_*width] = b'|';
                }
            }
        }
    }

    // show bubbles
    for bubble in bubbles {
        let x = bubble.borrow().x as usize;
        let y = bubble.borrow().y as usize;
        bmap[x+y*width] = b'o';
    }

    (width, height, bmap)
}


// keep track of unresolved constraints
//
// this requires a bit of a unique data structure, we need to be
// able to:
// 1. insert unique constraints
// 2. remove unique constraints
// 3. choose a random constraint with the lowest amount of
//    possiblities remaining
//
// unfortunately since we want this to be deterministic and random
// we can't just choose an arbitrary item from a BTreeMap/HashMap
//
#[derive(Debug)]
struct ConstraintSet {
    buckets: BTreeMap<u32, (
        HashMap<(usize, usize), usize>,
        Vec<(usize, usize)>
    )>
}

impl ConstraintSet {
    fn new() -> ConstraintSet {
        ConstraintSet{buckets: BTreeMap::new()}
    }

    fn insert(&mut self, c: u32, x: usize, y: usize) -> bool {
        let (ref mut map, ref mut bucket) = self.buckets.entry(c)
            .or_insert_with(|| (HashMap::new(), Vec::new()));
        match map.entry((x, y)) {
            hash_map::Entry::Occupied(_) => false,
            hash_map::Entry::Vacant(e) => {
                let i = bucket.len();
                bucket.push((x, y));
                e.insert(i);
                true
            }
        }
    }

    fn remove(&mut self, c: u32, x: usize, y: usize) -> bool {
        match self.buckets.entry(c) {
            btree_map::Entry::Occupied(mut e) => {
                let (ref mut map, ref mut bucket) = e.get_mut();
                match map.entry((x, y)) {
                    // one entry? drop bucket
                    hash_map::Entry::Occupied(_) if bucket.len() <= 1 => {
                        e.remove();
                        true
                    }
                    // more entries? need to swap-remove, this gets
                    // a bit messy since we also need to update the
                    // swapped entry's map entry
                    hash_map::Entry::Occupied(e) => {
                        let i = e.remove();
                        if i < bucket.len()-1 {
                            let (x_, y_) = bucket.pop().unwrap();
                            bucket[i] = (x_, y_);
                            map.insert((x_, y_), i);
                        }
                        true
                    }
                    hash_map::Entry::Vacant(_) => false,
                }
            },
            btree_map::Entry::Vacant(_) => false,
        }
    }

    fn pop(&mut self, prng: &mut Xorshift64) -> Option<(u32, usize, usize)> {
        match self.buckets.first_key_value() {
            Some((&c, (_, ref bucket))) => {
                // in case of tie, choose randomly
                let (x, y) = bucket[prng.range(0..bucket.len())];
                self.remove(c, x, y);
                Some((c, x, y))
            }
            None => None,
        }
    }
}

// create a tile map using wave function collapse
fn wfc_tile_map(
    prng: &mut Xorshift64,
    width: usize,
    height: usize,
    bubbles: &[Rc<RefCell<Bubble>>],
    scale: usize,
    attempts: usize,
) -> (bool, usize, usize, Vec<u8>) {
    // first create and fill a constraint map using our bubbles as a template,
    // note right now we only know which things are space and which things
    // aren't space
    let cwidth = width*scale;
    let cheight = height*scale;

    let mut cmap = vec![TILE_SPACE; cwidth*cheight];
    let mut success = false;

    'wfc: for _ in 0..attempts {
        // mark bubbles as available for all tiles
        for bubble in bubbles {
            let x = bubble.borrow().x as usize * scale;
            let y = bubble.borrow().y as usize * scale;
            let r = bubble.borrow().r * scale;
            for y_ in 0..cheight {
                for x_ in 0..cwidth {
                    if
                        distsq(
                            (x_ as isize, y_ as isize),
                            (x as isize, y as isize))
                            <= sq(r)
                    {
                        cmap[x_+y_*cwidth] = TILE_ALL & !TILE_SPACE;
                    }
                }
            }
        }

        // mark hallway walls as available for all tiles
        for bubble in bubbles {
            let x = bubble.borrow().x as usize * scale;
            let y = bubble.borrow().y as usize * scale;
            if let Some(parent) = &bubble.borrow().parent {
                let p_x = parent.borrow().x as usize * scale;
                let p_y = parent.borrow().y as usize * scale;
                for x_ in cmp::min(x, p_x) ..= cmp::max(x, p_x) {
                    for r in 0..(scale+1)/2 {
                        cmap[x_+(y+r)*cwidth] = TILE_ALL & !TILE_SPACE;
                        cmap[x_+(y-r)*cwidth] = TILE_ALL & !TILE_SPACE;
                    }
                }
                for y_ in cmp::min(y, p_y) ..= cmp::max(y, p_y) {
                    for r in 0..(scale+1)/2 {
                        cmap[(x+r)+y_*cwidth] = TILE_ALL & !TILE_SPACE;
                        cmap[(x-r)+y_*cwidth] = TILE_ALL & !TILE_SPACE;
                    }
                }
            }
        }

        // but hallways themselves as required floor
        for bubble in bubbles {
            let x = bubble.borrow().x as usize * scale;
            let y = bubble.borrow().y as usize * scale;
            if let Some(parent) = &bubble.borrow().parent {
                let p_x = parent.borrow().x as usize * scale;
                let p_y = parent.borrow().y as usize * scale;
                for x_ in cmp::min(x, p_x) ..= cmp::max(x, p_x) {
                    cmap[x_+y*cwidth] = TILE_FLOOR;
                    cmap[x_+y*cwidth] = TILE_FLOOR;
                }
                for y_ in cmp::min(y, p_y) ..= cmp::max(y, p_y) {
                    cmap[x+y_*cwidth] = TILE_FLOOR;
                    cmap[x+y_*cwidth] = TILE_FLOOR;
                }
            }
        }
        
        //let mut unresolved: BTreeSet<(u32, usize, usize)> = BTreeSet::new();
        let mut unresolved: ConstraintSet = ConstraintSet::new();

        // add all unresolved to our propagating set, these will be moved
        // into the unresolved tree after constraints are evaluated
        let mut propagating: Vec<(usize, usize)> = vec![];
        for y in 0..cheight {
            for x in 0..cwidth {
                let c = cmap[x+y*cwidth];
                if c.count_ones() > 1 {
                    propagating.push((x, y));
                }
            }
        }

        // core wfc algorithm
        loop {
            // propagate new constraints
            while let Some((x, y)) = propagating.pop() {
                let mut c = cmap[x+y*cwidth];

                // for each neighbor
                let mut constrain = |x_: usize, y_: usize, dir: Dir| {
                    let c_ = cmap[x_+y_*cwidth];

                    // what does our neighbor allow us to be?
                    let mut mask = 0;
                    for i in 0..TILES.len() {
                        if c_ & (1 << i) != 0 {
                            mask |= TILES[i].constraints.dir(dir.flip());
                        }
                    }
                    c &= mask;

                    // does any of our possibilities contradict our neighbor?
                    for i in 0..TILES.len() {
                        if
                            c & (1 << i) != 0
                                && TILES[i].constraints.dir(dir) & c_ == 0
                        {
                            c &= !(1 << i);
                        }
                    }
                };
                
                if x > 0 { constrain(x-1, y, Dir::W); }
                if y > 0 { constrain(x, y-1, Dir::N); }
                if x < cwidth-1 { constrain(x+1, y, Dir::E); }
                if y < cheight-1 { constrain(x, y+1, Dir::S); }

                // did we actually change anything?
                if cmap[x+y*cwidth] != c {
                    // update our map
                    let count = cmap[x+y*cwidth].count_ones();
                    let count_ = c.count_ones();
                    cmap[x+y*cwidth] = c;
                    // contradiction? abort the current wfc
                    if c == 0 {
                        continue 'wfc;
                    }
                    // move into different bucket
                    unresolved.remove(count, x, y);
                    unresolved.insert(count_, x, y);
                    // propagate constraints to our neighbors
                    if x > 0 { propagating.push((x-1, y)); }
                    if y > 0 { propagating.push((x, y-1)); }
                    if x < cwidth-1 { propagating.push((x+1, y)); }
                    if y < cheight-1 { propagating.push((x, y+1)); }
                }
            }

            // do we have unresolved constraints? choose the most-resolved
            match unresolved.pop(prng) {
                Some((_, x, y)) => {
                    // randomly assign it to one of its options
                    let mut c = cmap[x+y*cwidth];
                    debug_assert!(c.count_ones() > 0);
                    let choice = prng.range(0..c.count_ones() as usize);
                    // figure out which bit this actually is, kinda complicated
                    for _ in 0..choice {
                        c &= !(1 << (64-1-c.leading_zeros()));
                    }
                    c &= !((1 << (64-1-c.leading_zeros()))-1);

                    // update our map
                    cmap[x+y*cwidth] = c;
                    // propagate constraints to our neighbors
                    if x > 0 { propagating.push((x-1, y)); }
                    if y > 0 { propagating.push((x, y-1)); }
                    if x < cwidth-1 { propagating.push((x+1, y)); }
                    if y < cheight-1 { propagating.push((x, y+1)); }
                }
                None => {
                    success = true;
                    break 'wfc;
                }
            }
        }
    }

    // convert our constraint map into a tile map
    let mut tmap = vec![b'?'; cwidth*cheight*2];
    for y in 0..cheight {
        for x in 0..cwidth { 
            let ascii = match cmap[x+y*cwidth] {
                0 => b"!!",
                x if x.count_ones() == 1 => {
                    TILES[64-1-x.leading_zeros() as usize].ascii
                },
                _ => b"??",
            };
            tmap[(x+y*cwidth)*2 .. (x+y*cwidth)*2+2].copy_from_slice(ascii);
        }
    }

    (success, cwidth*2, cheight, tmap)
}



#[derive(Debug, StructOpt)]
#[structopt(rename_all="kebab")]
struct Opt {
    /// Size of your spacestation.
    size: usize,

    // TODO allow hex
    /// Optional seed for reproducibility.
    #[structopt(long)]
    seed: Option<u64>,

    /// Probability to expand a bubble.
    #[structopt(long, default_value="0.5")]
    bubble_p: f64,

    /// Probability to extend a hallway.
    #[structopt(long, default_value="0.5")]
    hallway_p: f64,

    /// Smallest possible bubble size.
    #[structopt(long, default_value="1")]
    smallest: usize,

    /// Required space between bubbles.
    #[structopt(long, default_value="1")]
    clearance: usize,

    /// Show a small map.
    #[structopt(short, long, alias="small")]
    small_map: bool,

    /// Show a bubble map.
    #[structopt(short, long, alias="bubble")]
    bubble_map: bool,

    /// Show a tiled map
    #[structopt(short, long, alias="tile")]
    tile_map: bool,

    /// Width of small map.
    #[structopt(long, default_value="8")]
    small_width: usize,

    /// Height of small map.
    #[structopt(long, default_value="8")]
    small_height: usize,

    /// Scale for tile map.
    #[structopt(long, default_value="3")]
    scale: usize,

    /// Number of attempts at constraining the tile map.
    ///
    /// If this fails, the tile set constraints may need
    /// to be relaxed.
    #[structopt(long, default_value="1000")]
    attempts: usize,
}

fn main() {
    // parse opts
    let mut opt = Opt::from_args();
    // if no maps are explicitly requested assume user wants all of them
    if !opt.small_map && !opt.bubble_map && !opt.tile_map {
        opt.small_map = true;
        opt.bubble_map = true;
        opt.tile_map = true;
    }
    let opt = opt;

    // initialize with either provided seed or actually random seed
    let mut prng = Xorshift64(opt.seed.unwrap_or_else(|| {
        rand::thread_rng().next_u64()
    }));
    println!("seed: 0x{:016x}", prng.0);

    // generate bubbles
    let (width, height, bubbles) = gen_bubbles(
        &mut prng,
        opt.size,
        opt.bubble_p,
        opt.hallway_p,
        opt.smallest,
        opt.clearance,
    );
    println!("generated: {}x{}", width, height);

    // render small map
    if opt.small_map {
        let (swidth, sheight, smap) = render_small_map(
            width,
            height,
            &bubbles,
            opt.small_width,
            opt.small_height,
        );

        for y in 0..sheight {
            for x in 0..swidth {
                print!("{}",
                    char::from_u32(smap[x+y*swidth] as u32).unwrap()
                );
            }
            println!();
        }
    }

    // render bubble map
    if opt.bubble_map {
        let (bwidth, bheight, bmap) = render_bubble_map(
            width,
            height,
            &bubbles,
        );

        for y in 0..bheight {
            for x in 0..bwidth {
                print!("{}",
                    char::from_u32(bmap[x+y*bwidth] as u32).unwrap()
                );
            }
            println!();
        }
    }

    // render tile map using wave function collapse
    if opt.tile_map {
        let (tsuccess, twidth, theight, tmap) = wfc_tile_map(
            &mut prng,
            width,
            height,
            &bubbles,
            opt.scale,
            opt.attempts,
        );
        println!("scaled: {}x{}", twidth, theight);

        for y in 0..theight {
            for x in 0..twidth {
                print!("{}",
                    char::from_u32(tmap[x+y*twidth] as u32).unwrap()
                );
            }
            println!();
        }

        if !tsuccess {
            println!("failed to resolve constraints after {} attempts",
                opt.attempts
            );
        }
    }
}
