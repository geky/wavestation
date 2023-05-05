#![allow(dead_code)]
#![allow(unused_imports)]

use structopt::StructOpt;
use rand::{self, RngCore};

use std::ops::RangeBounds;
use std::ops::Bound;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;
use std::collections::{BTreeMap, HashMap, hash_map, btree_map};
use std::io;
use std::num;
use std::str::FromStr;
use std::time::{Instant, Duration};

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

#[derive(Debug, Clone)]
struct WaveStation {
    // prng
    //
    // note we keep this here to ensure deterministic station generation,
    // no one else should call it!
    //
    seed: u64,
    prng: Xorshift64,

    // list of station bubbles
    //
    // order is maintained so that parent bubbles always precedes a bubble
    // in the list
    //
    // width/height effectively make a bounding box maintained around the
    // bubbles
    //
    size: usize,
    width: usize,
    height: usize,
    bubbles: Vec<Rc<RefCell<Bubble>>>,

    // constraint map optionally generated by wave-function collapse
    cwidth: usize,
    cheight: usize,
    cmap: Vec<u128>,
    delta: Vec<Rc<RefCell<Bubble>>>,

    // generation config
    bubble_p: f64,
    hallway_p: f64,
    smallest: usize,
    clearance: usize,
    scale: usize,
    attempts: u64,

    // stats
    bubble_cycles: u64,
    bubble_time: Duration,
    wfc_attempts: u64,
    wfc_cycles: u64,
    wfc_propagations: u64,
    wfc_time: Duration,
}

impl WaveStation {
    fn new(
        seed: Option<u64>,
        size: Option<usize>,
        bubble_p: f64,
        hallway_p: f64,
        smallest: usize,
        clearance: usize,
        scale: usize,
        attempts: u64,
    ) -> WaveStation {
        // initialize with either provided seed or actually random seed
        let seed = seed.unwrap_or_else(|| {
            rand::thread_rng().next_u64()
        });

        let mut self_ = WaveStation{
            seed: seed,
            prng: Xorshift64(seed),

            size: 0,
            width: 0,
            height: 0,
            // initialize with one bubble of a random size
            bubbles: vec![],

            cwidth: 0,
            cheight: 0,
            cmap: vec![],
            delta: vec![],

            bubble_p: bubble_p,
            hallway_p: hallway_p,
            smallest: smallest,
            clearance: clearance,
            scale: scale,
            attempts: attempts,

            bubble_cycles: 0,
            bubble_time: Duration::ZERO,
            wfc_attempts: 0,
            wfc_cycles: 0,
            wfc_propagations: 0,
            wfc_time: Duration::ZERO,
        };

        // initialize with one bubble of a random size
        let bubble = Rc::new(RefCell::new(Bubble{
            x: 0,
            y: 0,
            r: smallest + self_.prng.poisson(bubble_p),
            parent: None,
        }));
        self_.bubbles.push(bubble.clone());
        self_.size += bubble.borrow().r;

        // generate requested size, note we may overshoot
        if let Some(size) = size {
            if size > self_.size {
                self_.gen_bubbles(size - self_.size);
            }
        }

        self_.center();
        self_
    }

    fn gen_bubbles(&mut self, delta: usize) {
        let start = Instant::now();

        let size = self.size + delta;
        while self.size < size {
            self.bubble_cycles += 1;
            // choose a bubble
            let parent = &self.bubbles[self.prng.range(0..self.bubbles.len())];
            // choose a direction
            let (dir_x, dir_y) = match self.prng.range(0..4) {
                0 => (0, 1),
                1 => (1, 0),
                2 => (0, -1),
                3 => (-1, 0),
                _ => unreachable!(),
            };
            // choose a size
            let r = self.smallest + self.prng.poisson(self.bubble_p);

            // calculate new position
            let hallway = self.clearance + self.prng.poisson(self.hallway_p);
            let x = parent.borrow().x
                + dir_x*((parent.borrow().r + r + hallway) as isize);
            let y = parent.borrow().y
                + dir_y*((parent.borrow().r + r + hallway) as isize);

            // but wait, is there a collision?
            let mut collision = false;
            for bubble in &self.bubbles {
                if Rc::ptr_eq(&bubble, parent) {
                    continue;
                }

                // check bubble collision
                if
                    distsq((x, y), (bubble.borrow().x, bubble.borrow().y))
                        <= sq(r + bubble.borrow().r + self.clearance)
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
            self.bubbles.push(bubble.clone());
            self.size += r;

            // keep track of new bubbles if we have a constraint map
            if self.cmap.len() > 0 {
                self.delta.push(bubble);
            }
        }

        self.center();

        let stop = Instant::now();
        self.bubble_time += stop.duration_since(start);
    }

    fn center(&mut self) {
        // find bounds
        let (mut lower_x, mut lower_y) = (0, 0);
        let (mut upper_x, mut upper_y) = (1, 1);
        for bubble in &self.bubbles {
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
        self.width = (upper_x+1 - lower_x) as usize;
        self.height = (upper_y+1 - lower_y) as usize;

        // shift all bubbles into the range 0,0 => width,height
        for bubble in &self.bubbles {
            bubble.borrow_mut().x -= lower_x;
            bubble.borrow_mut().y -= lower_y;
        }

        if self.cmap.len() > 0 {
            // adjust our constraint map as necessary, filling
            // in uninitialized constraints with space for now
            let cwidth = self.width*self.scale;
            let cheight = self.height*self.scale;
            let mut cmap = vec![TILE_SPACE; cwidth*cheight];

            debug_assert!(lower_x < 0);
            debug_assert!(lower_y < 0);
            for y in 0..self.cheight {
                for x in 0..self.cwidth {
                    cmap[
                        (x as isize-lower_x*self.scale as isize) as usize
                        + (y as isize-lower_y*self.scale as isize) as usize
                            *cwidth
                    ] = self.cmap[x+y*self.cwidth];
                }
            }

            self.cwidth = cwidth;
            self.cheight = cheight;
            self.cmap = cmap;
        }
    }

    // render small map at a requested size
    fn render_small_map(
        &self,
        swidth: usize,
        sheight: usize,
    ) -> (usize, usize, Vec<u8>) {
        let mut smap = vec![b' '; swidth*sheight];
        let scale_x = swidth as f64 / self.width as f64;
        let scale_y = sheight as f64 / self.height as f64;

        // show hallways
        for bubble in &self.bubbles {
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
        for bubble in &self.bubbles {
            let x = (bubble.borrow().x as f64 * scale_x) as usize;
            let y = (bubble.borrow().y as f64 * scale_y) as usize;
            smap[x+y*swidth] = b'o';
        }

        (swidth, sheight, smap)
    }

    // render bubble map
    fn render_bubble_map(&self) -> (usize, usize, Vec<u8>) {
        let mut bmap = vec![b' '; self.width*self.height];

        // show bubbles
        for bubble in &self.bubbles {
            let x = bubble.borrow().x as usize;
            let y = bubble.borrow().y as usize;
            let r = bubble.borrow().r;
            for y_ in 0..self.height {
                for x_ in 0..self.width {
                    if
                        distsq((x_ as isize, y_ as isize), (x as isize, y as isize))
                            <= sq(r)
                    {
                        bmap[x_+y_*self.width] = b'.';
                    }
                }
            }
        }

        // show hallways
        for bubble in &self.bubbles {
            let x = bubble.borrow().x as usize;
            let y = bubble.borrow().y as usize;
            if let Some(parent) = &bubble.borrow().parent {
                let p_x = parent.borrow().x as usize;
                let p_y = parent.borrow().y as usize;
                for x_ in cmp::min(x, p_x) ..= cmp::max(x, p_x) {
                    if bmap[x_+y*self.width] == b'|' {
                        bmap[x_+y*self.width] = b'+';
                    } else {
                        bmap[x_+y*self.width] = b'-';
                    }
                }
                for y_ in cmp::min(y, p_y) ..= cmp::max(y, p_y) {
                    if bmap[x+y_*self.width] == b'-' {
                        bmap[x+y_*self.width] = b'+';
                    } else {
                        bmap[x+y_*self.width] = b'|';
                    }
                }
            }
        }

        // show bubbles
        for bubble in &self.bubbles {
            let x = bubble.borrow().x as usize;
            let y = bubble.borrow().y as usize;
            bmap[x+y*self.width] = b'o';
        }

        (self.width, self.height, bmap)
    }
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
#[derive(Debug, Clone)]
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

impl WaveStation {
    // evaluate any unresolved constraints in our constraint map
    // with wave-function collapse
    fn wfc(&mut self) -> bool {
        let start = Instant::now();

        // lazily initialize our initial constraint map since wfc is
        // expensive and may not be used, by default all constraints
        // should be space
        let delta = match self.cmap.len() {
            0 =>  {
                self.cwidth = self.width*self.scale;
                self.cheight = self.height*self.scale;
                self.cmap = vec![TILE_SPACE; self.cwidth*self.cheight];
                &self.bubbles
            },
            _ => {
                &self.delta
            }
        };

        // update our constraint map with changes to bubble map

        // TODO we need to consider all
        // 1. new bubbles
        // 2. their parents
        // 3. any hallways they collide with
        // 4. any bubbles their hallway collides with

        // collect any new bubbles
//        for bubble in delta {
//            pending.push(bubble.clone());
//            if let Some(parent) = &bubble.borrow().parent {
//                pending.push(parent.clone());
//            }
//        }

        // collect any bubbles our new hallways collide with, this
        // should include our new bubble and our parent
        let mut delta_bubbles: Vec<Rc<RefCell<Bubble>>> = vec![];
        for bubble in delta {
            if let Some(parent) = &bubble.borrow().parent {
                let a_x = bubble.borrow().x;
                let a_y = bubble.borrow().y;
                let b_x = parent.borrow().x;
                let b_y = parent.borrow().y;
                for bubble_ in &self.bubbles {
                    let x = bubble_.borrow().x;
                    let y = bubble_.borrow().y;
                    let r = bubble_.borrow().r;
                    let mut collides = false;

                    // TODO cleanup?
                    // naive line/circle collision detection
                    //
                    // this could be made faster with a bit of math,
                    // but math is hard
                    for a_x_ in cmp::min(a_x, b_x) ..= cmp::max(a_x, b_x) {
                        if
                            distsq(
                                (x as isize, y as isize),
                                (a_x_ as isize, a_y as isize)
                            ) <= sq(r)
                        {
                            collides = true;
                            break;
                        }
                    }
                    for a_y_ in cmp::min(a_y, b_y) ..= cmp::max(a_y, b_y) {
                        if
                            distsq(
                                (x as isize, y as isize),
                                (a_x as isize, a_y_ as isize)
                            ) <= sq(r)
                        {
                            collides = true;
                            break;
                        }
                    }

                    if collides {
                        delta_bubbles.push(bubble_.clone());
                    }
                }
            }
        }

        // collect any hallways our bubbles collide with
        let mut delta_hallways: Vec<((usize, usize), (usize, usize))> = vec![];
        for bubble in &delta_bubbles {
            let x = bubble.borrow().x;
            let y = bubble.borrow().y;
            let r = bubble.borrow().r;
            for bubble_ in &self.bubbles {
                if let Some(parent) = &bubble_.borrow().parent {
                    let a_x = bubble_.borrow().x;
                    let a_y = bubble_.borrow().y;
                    let b_x = parent.borrow().x;
                    let b_y = parent.borrow().y;
                    let mut collides = false;

                    // TODO cleanup?
                    // naive line/circle collision detection
                    //
                    // this could be made faster with a bit of math,
                    // but math is hard
                    for a_x_ in cmp::min(a_x, b_x) ..= cmp::max(a_x, b_x) {
                        if
                            distsq(
                                (x as isize, y as isize),
                                (a_x_ as isize, a_y as isize)
                            ) <= sq(r)
                        {
                            collides = true;
                            break;
                        }
                    }
                    for a_y_ in cmp::min(a_y, b_y) ..= cmp::max(a_y, b_y) {
                        if
                            distsq(
                                (x as isize, y as isize),
                                (a_x as isize, a_y_ as isize)
                            ) <= sq(r)
                        {
                            collides = true;
                            break;
                        }
                    }

                    if collides {
                        delta_hallways.push((
                            (a_x as usize, a_y as usize),
                            (b_x as usize, b_y as usize)
                        ));
                    }
                }
            }
        }
        

        // mark bubbles as not space
        for bubble in &delta_bubbles {
            let x = bubble.borrow().x as usize * self.scale;
            let y = bubble.borrow().y as usize * self.scale;
            let r = bubble.borrow().r * self.scale;
            for y_ in 0..self.cheight {
                for x_ in 0..self.cwidth {
                    if
                        distsq(
                            (x_ as isize, y_ as isize),
                            (x as isize, y as isize))
                            <= sq(r)
                    {
                        self.cmap[x_+y_*self.cwidth]
                            = TILE_ALL & !TILE_SPACE;
                    }
                }
            }
        }

        // mark hallway walls as not space
        for ((a_x, a_y), (b_x, b_y)) in &delta_hallways {
            let a_x = a_x * self.scale;
            let a_y = a_y * self.scale;
            let b_x = b_x * self.scale;
            let b_y = b_y * self.scale;

            for a_x_ in cmp::min(a_x, b_x) ..= cmp::max(a_x, b_x) {
                for r in 0..(self.scale+1)/2 {
                    self.cmap[a_x_+(a_y+r)*self.cwidth]
                        = TILE_ALL & !TILE_SPACE;
                    self.cmap[a_x_+(a_y-r)*self.cwidth]
                        = TILE_ALL & !TILE_SPACE;
                }
            }
            for a_y in cmp::min(a_y, b_y) ..= cmp::max(a_y, b_y) {
                for r in 0..(self.scale+1)/2 {
                    self.cmap[(a_x+r)+a_y*self.cwidth]
                        = TILE_ALL & !TILE_SPACE;
                    self.cmap[(a_x-r)+a_y*self.cwidth]
                        = TILE_ALL & !TILE_SPACE;
                }
            }
        }

        // but hallways themselves as required floor
        for ((a_x, a_y), (b_x, b_y)) in &delta_hallways {
            let a_x = a_x * self.scale;
            let a_y = a_y * self.scale;
            let b_x = b_x * self.scale;
            let b_y = b_y * self.scale;

            for a_x_ in cmp::min(a_x, b_x) ..= cmp::max(a_x, b_x) {
                self.cmap[a_x_+a_y*self.cwidth] = TILE_FLOOR;
                self.cmap[a_x_+a_y*self.cwidth] = TILE_FLOOR;
            }
            for a_y in cmp::min(a_y, b_y) ..= cmp::max(a_y, b_y) {
                self.cmap[a_x+a_y*self.cwidth] = TILE_FLOOR;
                self.cmap[a_x+a_y*self.cwidth] = TILE_FLOOR;
            }
        }

//        for bubble in delta {
//            let x = bubble.borrow().x as usize * self.scale;
//            let y = bubble.borrow().y as usize * self.scale;
//            let r = bubble.borrow().r * self.scale;
//            for y_ in 0..self.cheight {
//                for x_ in 0..self.cwidth {
//                    if
//                        distsq(
//                            (x_ as isize, y_ as isize),
//                            (x as isize, y as isize))
//                            <= sq(r)
//                    {
//                        self.cmap[x_+y_*self.cwidth]
//                            = TILE_ALL & !TILE_SPACE;
//                    }
//                }
//            }
//
//            // also mark parent bubbles as not space, needed
//            // if we're generating incrementally
//            //
//            // basically we need to throw out our parent's state
//            // to construct a hallway
//            if let Some(parent) = &bubble.borrow().parent {
//                let x = parent.borrow().x as usize * self.scale;
//                let y = parent.borrow().y as usize * self.scale;
//                let r = parent.borrow().r * self.scale;
//                for y_ in 0..self.cheight {
//                    for x_ in 0..self.cwidth {
//                        if
//                            distsq(
//                                (x_ as isize, y_ as isize),
//                                (x as isize, y as isize))
//                                <= sq(r)
//                        {
//                            self.cmap[x_+y_*self.cwidth]
//                                = TILE_ALL & !TILE_SPACE;
//                        }
//                    }
//                }
//            }
//        }
//
//        // mark hallway walls as not space
//        //
//        // note we need to consider all hallways that collide with our bubbles
//        for bubble in &self.bubbles {
//            let x = bubble.borrow().x as usize * self.scale;
//            let y = bubble.borrow().y as usize * self.scale;
//            if let Some(parent) = &bubble.borrow().parent {
//                let p_x = parent.borrow().x as usize * self.scale;
//                let p_y = parent.borrow().y as usize * self.scale;
//                for x_ in cmp::min(x, p_x) ..= cmp::max(x, p_x) {
//                    for r in 0..(self.scale+1)/2 {
//                        self.cmap[x_+(y+r)*self.cwidth]
//                            = TILE_ALL & !TILE_SPACE;
//                        self.cmap[x_+(y-r)*self.cwidth]
//                            = TILE_ALL & !TILE_SPACE;
//                    }
//                }
//                for y_ in cmp::min(y, p_y) ..= cmp::max(y, p_y) {
//                    for r in 0..(self.scale+1)/2 {
//                        self.cmap[(x+r)+y_*self.cwidth]
//                            = TILE_ALL & !TILE_SPACE;
//                        self.cmap[(x-r)+y_*self.cwidth]
//                            = TILE_ALL & !TILE_SPACE;
//                    }
//                }
//            }
//        }
//
//        // but hallways themselves as required floor
//        //
//        // note we need to consider all hallways that collide with our bubbles
//        for bubble in &self.bubbles {
//            let x = bubble.borrow().x as usize * self.scale;
//            let y = bubble.borrow().y as usize * self.scale;
//            self.cmap[x+y*self.cwidth] = TILE_FLOOR;
//            if let Some(parent) = &bubble.borrow().parent {
//                let p_x = parent.borrow().x as usize * self.scale;
//                let p_y = parent.borrow().y as usize * self.scale;
//                for x_ in cmp::min(x, p_x) ..= cmp::max(x, p_x) {
//                    self.cmap[x_+y*self.cwidth] = TILE_FLOOR;
//                    self.cmap[x_+y*self.cwidth] = TILE_FLOOR;
//                }
//                for y_ in cmp::min(y, p_y) ..= cmp::max(y, p_y) {
//                    self.cmap[x+y_*self.cwidth] = TILE_FLOOR;
//                    self.cmap[x+y_*self.cwidth] = TILE_FLOOR;
//                }
//            }
//        }

        // reset our delta, these bubbles are now at least represented
        // in our constraint map
        self.delta.clear();

        // TODO rm me
        let (twidth, theight, tmap) = self.render_tile_map();

        for y in 0..theight {
            for x in 0..twidth {
                print!("{}",
                    char::from_u32(tmap[x+y*twidth] as u32).unwrap()
                );
            }
            println!();
        }


        // copy our constraint map for the core wfc algorithm, this allows
        // us to quickly revert failed attempts
        let init_cmap = self.cmap.clone();

        // figure out what we actually need to resolve
        let mut init_unresolved: Vec<(usize, usize)> = vec![];
        for y in 0..self.cheight {
            for x in 0..self.cwidth {
                let c = self.cmap[x+y*self.cwidth];
                if c.count_ones() > 1 {
                    init_unresolved.push((x, y));
                }
            }
        }
        let init_unresolved = init_unresolved;

        let mut success = false;
        self.wfc_attempts = 0;

        'wfc: for _ in 0..self.attempts {
            self.wfc_attempts += 1;

            // reset to initial constraint map
            self.cmap = init_cmap.clone();

            // keep track of all unresolved constraints
            let mut unresolved: ConstraintSet = ConstraintSet::new();

            // add all unresolved to our propagating set, these will be moved
            // into the unresolved tree after constraints are evaluated
            let mut propagating: Vec<(usize, usize)> = init_unresolved.clone();

            // core wfc algorithm
            loop {
                self.wfc_cycles += 1;

                // propagate new constraints
                while let Some((x, y)) = propagating.pop() {
                    self.wfc_propagations += 1;
                    let mut c = self.cmap[x+y*self.cwidth];

                    // for each neighbor
                    let mut constrain = |x_: usize, y_: usize, dir: Dir| {
                        let c_ = self.cmap[x_+y_*self.cwidth];

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
                    if x < self.cwidth-1 { constrain(x+1, y, Dir::E); }
                    if y < self.cheight-1 { constrain(x, y+1, Dir::S); }

                    // did we actually change anything?
                    if self.cmap[x+y*self.cwidth] != c {
                        // update our map
                        let count = self.cmap[x+y*self.cwidth].count_ones();
                        let count_ = c.count_ones();
                        self.cmap[x+y*self.cwidth] = c;
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
                        if x < self.cwidth-1 { propagating.push((x+1, y)); }
                        if y < self.cheight-1 { propagating.push((x, y+1)); }
                    }
                }

                // do we have unresolved constraints? choose the most-resolved
                match unresolved.pop(&mut self.prng) {
                    Some((_, x, y)) => {
                        // randomly assign it to one of its options
                        let mut c = self.cmap[x+y*self.cwidth];
                        debug_assert!(c.count_ones() > 0);
                        let choice = self.prng.range(
                            0..c.count_ones() as usize
                        );
                        // figure out which bit this actually is, kinda complicated
                        for _ in 0..choice {
                            c &= !(1 << (128-1-c.leading_zeros()));
                        }
                        c &= !((1 << (128-1-c.leading_zeros()))-1);

                        // update our map
                        self.cmap[x+y*self.cwidth] = c;
                        // propagate constraints to our neighbors
                        if x > 0 { propagating.push((x-1, y)); }
                        if y > 0 { propagating.push((x, y-1)); }
                        if x < self.cwidth-1 { propagating.push((x+1, y)); }
                        if y < self.cheight-1 { propagating.push((x, y+1)); }
                    }
                    None => {
                        success = true;
                        break 'wfc;
                    }
                }
            }
        }

        let stop = Instant::now();
        self.wfc_time += stop.duration_since(start);

        success
    }

    // render tile map
    fn render_tile_map(&self) -> (usize, usize, Vec<u8>) {
        // convert our constraint map into a tile map
        let mut tmap = vec![b'?'; self.cwidth*self.cheight*2];
        for y in 0..self.cheight {
            for x in 0..self.cwidth { 
                let ascii = match self.cmap[x+y*self.cwidth] {
                    0 => b"!!",
                    x if x.count_ones() == 1 => {
                        TILES[128-1-x.leading_zeros() as usize].ascii
                    },
                    _ => b"??",
                };
                tmap[(x+y*self.cwidth)*2 .. (x+y*self.cwidth)*2+2]
                    .copy_from_slice(ascii);
            }
        }

        (self.cwidth*2, self.cheight, tmap)
    }
}


fn parse_u64(s: &str) -> Result<u64, num::ParseIntError> {
    if s.starts_with("0x") {
        Ok(u64::from_str_radix(&s[2..], 16)?)
    } else if s.starts_with("0o") {
        Ok(u64::from_str_radix(&s[2..], 8)?)
    } else if s.starts_with("0b") {
        Ok(u64::from_str_radix(&s[2..], 2)?)
    } else {
        Ok(u64::from_str(s)?)
    }
}

fn parse_usize(s: &str) -> Result<usize, num::ParseIntError> {
    if s.starts_with("0x") {
        Ok(usize::from_str_radix(&s[2..], 16)?)
    } else if s.starts_with("0o") {
        Ok(usize::from_str_radix(&s[2..], 8)?)
    } else if s.starts_with("0b") {
        Ok(usize::from_str_radix(&s[2..], 2)?)
    } else {
        Ok(usize::from_str(s)?)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all="kebab")]
struct Opt {
    /// Size of your spacestation.
    size: usize,

    /// Optional seed for reproducibility.
    #[structopt(long, parse(try_from_str=parse_u64))]
    seed: Option<u64>,

    /// Probability to expand a bubble.
    #[structopt(long, default_value="0.5")]
    bubble_p: f64,

    /// Probability to extend a hallway.
    #[structopt(long, default_value="0.5")]
    hallway_p: f64,

    /// Smallest possible bubble size.
    #[structopt(long, default_value="1", parse(try_from_str=parse_usize))]
    smallest: usize,

    /// Required space between bubbles.
    #[structopt(long, default_value="1", parse(try_from_str=parse_usize))]
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
    #[structopt(long, default_value="8", parse(try_from_str=parse_usize))]
    small_width: usize,

    /// Height of small map.
    #[structopt(long, default_value="8", parse(try_from_str=parse_usize))]
    small_height: usize,

    /// Scale for tile map.
    #[structopt(long, default_value="3", parse(try_from_str=parse_usize))]
    scale: usize,

    /// Number of attempts at constraining the tile map before giving up.
    ///
    /// If this fails, consider also relaxing the tile set constraints.
    #[structopt(long, default_value="1000", parse(try_from_str=parse_u64))]
    attempts: u64,

    /// How much station size to generate at once
    ///
    /// Larger values may increase performance, but at a risk of increasing
    /// wfc failure
    #[structopt(long, default_value="1", parse(try_from_str=parse_usize))]
    chunk_size: usize,
}

fn main() {
    // parse opts
    let mut opt = Opt::from_args();
    // if no maps are explicitly requested, assume a bubble map
    //
    // mostly because this one is my favorite
    if !opt.small_map && !opt.bubble_map && !opt.tile_map {
        opt.bubble_map = true;
    }
    let opt = opt;

    // create our wavestation, this class does most of the work
    let mut ws = WaveStation::new(
        opt.seed,
        None,
        opt.bubble_p,
        opt.hallway_p,
        opt.smallest,
        opt.clearance,
        opt.scale,
        opt.attempts,
    );
    println!("seed: 0x{:016x}", ws.seed);

    // generate in chunks to avoid wfc failures
    let mut success = true;
    loop {
        // generate bubbles
        if opt.size > ws.size {
            ws.gen_bubbles(cmp::min(opt.chunk_size, opt.size-ws.size));
        }

        if opt.tile_map {
            // perform wfc on any new bubbles
            //
            // new bubbles may come from initialization!
            success = ws.wfc();
            if !success {
                break;
            }
        }

        if ws.size >= opt.size {
            break;
        }
    }

    // one last wfc to make sure things are cleaned up

    // print stats
    println!("gen: {}x{} cells, {} bubbles",
        ws.width,
        ws.height,
        ws.bubbles.len()
    );
    println!("in: {} cycles, {:?}",
        ws.bubble_cycles,
        ws.bubble_time
    );
    println!("wfc: {}x{} tiles, {} constraints",
        ws.cwidth, ws.cheight,
        // note each tile has 4 directional constraints
        TILES.len()*4
    );
    println!("in: {}/{} attempts, {} cycles, {} propagations, {:?}",
        ws.wfc_attempts,
        opt.attempts,
        ws.wfc_cycles,
        ws.wfc_propagations,
        ws.wfc_time,
    );

    // render small map
    if opt.small_map {
        let (swidth, sheight, smap) = ws.render_small_map(
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
        let (bwidth, bheight, bmap) = ws.render_bubble_map();

        for y in 0..bheight {
            for x in 0..bwidth {
                print!("{}",
                    char::from_u32(bmap[x+y*bwidth] as u32).unwrap()
                );
            }
            println!();
        }
    }

    // render tile
    if opt.tile_map {
        let (twidth, theight, tmap) = ws.render_tile_map();

        for y in 0..theight {
            for x in 0..twidth {
                print!("{}",
                    char::from_u32(tmap[x+y*twidth] as u32).unwrap()
                );
            }
            println!();
        }
    }

    if !success {
        println!("failed to resolve constraints after {} attempts!",
            opt.attempts
        );
    }
}
