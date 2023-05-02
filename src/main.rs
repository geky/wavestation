#![allow(dead_code)]

use structopt::StructOpt;
use rand::{self, RngCore};

use std::ops::RangeBounds;
use std::ops::Bound;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;


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
        let r = 1 + prng.poisson(bubble_p);

        // calculate new position
        let hallway = 1 + prng.poisson(hallway_p);
        let x = parent.borrow().x + dir_x*((parent.borrow().r + r + hallway) as isize);
        let y = parent.borrow().y + dir_y*((parent.borrow().r + r + hallway) as isize);

        // but wait, is there a collision?
        let mut collision = false;
        for bubble in &bubbles {
            if Rc::ptr_eq(&bubble, parent) {
                continue;
            }

            // check bubble collision
            if
                distsq((x, y), (bubble.borrow().x, bubble.borrow().y))
                    <= sq(r + bubble.borrow().r)
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
    let width = (upper_x - lower_x) as usize;
    let height = (upper_y - lower_y) as usize;

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
    small_width: usize,
    small_height: usize,
) -> Vec<u8> {
    let mut smap = vec![b' '; small_width*small_height];
    let scale_x = small_width as f64 / width as f64;
    let scale_y = small_height as f64 / height as f64;

    // show hallways
    for bubble in bubbles {
        let x = (bubble.borrow().x as f64 * scale_x) as usize;
        let y = (bubble.borrow().y as f64 * scale_y) as usize;
        if let Some(parent) = &bubble.borrow().parent {
            let p_x = (parent.borrow().x as f64 * scale_x) as usize;
            let p_y = (parent.borrow().y as f64 * scale_y) as usize;
            for x_ in cmp::min(x, p_x) ..= cmp::max(x, p_x) {
                if smap[x_+y*small_width] == b'|' {
                    smap[x_+y*small_width] = b'+';
                } else {
                    smap[x_+y*small_width] = b'-';
                }
            }
            for y_ in cmp::min(y, p_y) ..= cmp::max(y, p_y) {
                if smap[x+y_*small_width] == b'-' {
                    smap[x+y_*small_width] = b'+';
                } else {
                    smap[x+y_*small_width] = b'|';
                }
            }
        }
    }

    // show bubbles
    for bubble in bubbles {
        let x = (bubble.borrow().x as f64 * scale_x) as usize;
        let y = (bubble.borrow().y as f64 * scale_y) as usize;
        smap[x+y*small_width] = b'o';
    }

    smap
}

// render bubble map
fn render_bubble_map(
    width: usize,
    height: usize,
    bubbles: &[Rc<RefCell<Bubble>>],
) -> Vec<u8> {
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

    bmap
}







#[derive(Debug, StructOpt)]
#[structopt(rename_all="kebab")]
struct Opt {
    /// Size of your spacestation.
    size: usize,

    /// Optional seed for reproducibility.
    #[structopt(long)]
    seed: Option<u64>,

    /// Probability to expand a bubble.
    #[structopt(long, default_value="0.5")]
    bubble_p: f64,

    /// Probability to extend a hallway.
    #[structopt(long, default_value="0.5")]
    hallway_p: f64,

    /// Show a small map.
    #[structopt(short, long, alias="small")]
    small_map: bool,

    /// Show a bubble map.
    #[structopt(short, long, alias="bubble")]
    bubble_map: bool,

    /// Width of small map.
    #[structopt(long, default_value="8")]
    small_width: usize,

    /// Height of small map.
    #[structopt(long, default_value="8")]
    small_height: usize,
}

fn main() {
    // parse opts
    let mut opt = Opt::from_args();
    // if no maps are explicitly requested assume user wants all of them
    if !opt.small_map && !opt.bubble_map {
        opt.small_map = true;
        opt.bubble_map = true;
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
    );
    println!("widthxheight: {}x{}", width, height);

    // render small map
    if opt.small_map {
        let smap = render_small_map(
            width, height, &bubbles,
            opt.small_width, opt.small_height
        );

        for y in 0..opt.small_height {
            for x in 0..opt.small_width {
                print!("{}",
                    char::from_u32(smap[x+y*opt.small_width] as u32).unwrap()
                );
            }
            println!();
        }
    }

    // render bubble map, note we need this for wfc
    let bmap = render_bubble_map(width, height, &bubbles);
    if opt.bubble_map {
        for y in 0..height {
            for x in 0..width {
                print!("{}",
                    char::from_u32(bmap[x+y*width] as u32).unwrap()
                );
            }
            println!();
        }
    }
}
