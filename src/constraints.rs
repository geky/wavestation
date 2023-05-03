

// these types are how we define constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn flip(self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Constraints {
    pub n: u64,
    pub e: u64,
    pub s: u64,
    pub w: u64,
}

impl Constraints {
    const NONE: Constraints = Constraints::same(0);
    const ALL:  Constraints = Constraints::same(u64::MAX);

    const fn same(c: u64) -> Constraints {
        Constraints{n: c, e: c, s: c, w: c}
    }

    pub fn dir(&self, dir: Dir) -> u64 {
        match dir {
            Dir::N => self.n,
            Dir::E => self.e,
            Dir::S => self.s,
            Dir::W => self.w,
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    pub name: &'static str,
    pub ascii: &'static [u8],
    pub constraints: Constraints,
}

impl Tile {
    const fn new(
        name: &'static str,
        ascii: &'static [u8],
        constraints: Constraints
    ) -> Tile {
        Tile{
            name: name,
            ascii: ascii,
            constraints: constraints
        }
    }
}


// each tile gets a bit in a u64, this is also its index in the following array
//
// unfortunately Rust doesn't make this easy for us, and I'm too lazy to
// write a proc_macro
//
pub const TILE_SPACE        : u64 = 1 << 0;

pub const TILE_FLOOR        : u64 = 1 << 1;

pub const TILE_N_WALL       : u64 = 1 << 2;
pub const TILE_E_WALL       : u64 = 1 << 3;
pub const TILE_S_WALL       : u64 = 1 << 4;
pub const TILE_W_WALL       : u64 = 1 << 5;

pub const TILE_NE_WALL      : u64 = 1 << 6;
pub const TILE_SE_WALL      : u64 = 1 << 7;
pub const TILE_SW_WALL      : u64 = 1 << 8;
pub const TILE_NW_WALL      : u64 = 1 << 9;

pub const TILE_NE_WALL2     : u64 = 1 << 10;
pub const TILE_SE_WALL2     : u64 = 1 << 11;
pub const TILE_SW_WALL2     : u64 = 1 << 12;
pub const TILE_NW_WALL2     : u64 = 1 << 13;

pub const TILE_DEGENERATE_N : u64 = 1 << 14;
pub const TILE_DEGENERATE_E : u64 = 1 << 15;
pub const TILE_DEGENERATE_S : u64 = 1 << 16;
pub const TILE_DEGENERATE_W : u64 = 1 << 17;

//pub const TILE_BARREL_N     : u64 = 1 << 18;
//pub const TILE_BARREL_E     : u64 = 1 << 19;
//pub const TILE_BARREL_S     : u64 = 1 << 20;
//pub const TILE_BARREL_W     : u64 = 1 << 21;

// some common constraints
pub const TILE_SPACEISH: u64
    = TILE_SPACE
    | TILE_DEGENERATE_N
    | TILE_DEGENERATE_E
    | TILE_DEGENERATE_S
    | TILE_DEGENERATE_W;

pub const TILE_FLOORISH: u64
    = TILE_FLOOR;
//    | TILE_BARREL_N
//    | TILE_BARREL_E
//    | TILE_BARREL_S
//    | TILE_BARREL_W;

//pub const TILE_BARREL: u64
//    = TILE_BARREL_N
//    | TILE_BARREL_E
//    | TILE_BARREL_S
//    | TILE_BARREL_W;

// all tiles in our system
pub const TILES: [Tile; 18] = [
    // space
    Tile::new("space", b"  ", Constraints::ALL),

    // inside things
    Tile::new("floor", b"  ", Constraints{
        n: TILE_N_WALL|TILE_NE_WALL2|TILE_NW_WALL2|TILE_FLOORISH,
        e: TILE_E_WALL|TILE_NE_WALL2|TILE_SE_WALL2|TILE_FLOORISH,
        s: TILE_S_WALL|TILE_SE_WALL2|TILE_SW_WALL2|TILE_FLOORISH,
        w: TILE_W_WALL|TILE_NW_WALL2|TILE_SW_WALL2|TILE_FLOORISH
    }),

    // walls
    Tile::new("n-wall", b"--", Constraints{n: TILE_SPACEISH,    e: !TILE_SPACEISH,  s: !TILE_SPACEISH,  w: !TILE_SPACEISH}),
    Tile::new("e-wall", b"| ", Constraints{n: !TILE_SPACEISH,   e: TILE_SPACEISH,   s: !TILE_SPACEISH,  w: !TILE_SPACEISH}),
    Tile::new("s-wall", b"--", Constraints{n: !TILE_SPACEISH,   e: !TILE_SPACEISH,  s: TILE_SPACEISH,   w: !TILE_SPACEISH}),
    Tile::new("w-wall", b" |", Constraints{n: !TILE_SPACEISH,   e: !TILE_SPACEISH,  s: !TILE_SPACEISH,  w: TILE_SPACEISH}),

    Tile::new("ne-wall", b". ", Constraints{n: TILE_SPACEISH,   e: TILE_SPACEISH,   s: !TILE_SPACEISH,  w: !TILE_SPACEISH}),
    Tile::new("se-wall", b"' ", Constraints{n: !TILE_SPACEISH,  e: TILE_SPACEISH,   s: TILE_SPACEISH,   w: !TILE_SPACEISH}),
    Tile::new("sw-wall", b" '", Constraints{n: !TILE_SPACEISH,  e: !TILE_SPACEISH,  s: TILE_SPACEISH,   w: TILE_SPACEISH}),
    Tile::new("nw-wall", b" .", Constraints{n: TILE_SPACEISH,   e: !TILE_SPACEISH,  s: !TILE_SPACEISH,  w: TILE_SPACEISH}),

    Tile::new("ne-wall2", b" '", Constraints{n: TILE_NE_WALL|TILE_E_WALL,    e: TILE_NE_WALL|TILE_N_WALL,    s: !TILE_SPACEISH,              w: !TILE_SPACEISH}),
    Tile::new("se-wall2", b" .", Constraints{n: !TILE_SPACEISH,              e: TILE_SE_WALL|TILE_S_WALL,    s: TILE_SE_WALL|TILE_E_WALL,    w: !TILE_SPACEISH}),
    Tile::new("sw-wall2", b". ", Constraints{n: !TILE_SPACEISH,              e: !TILE_SPACEISH,              s: TILE_SW_WALL|TILE_W_WALL,    w: TILE_SW_WALL|TILE_S_WALL}),
    Tile::new("nw-wall2", b"' ", Constraints{n: TILE_NW_WALL|TILE_W_WALL,    e: !TILE_SPACEISH,              s: !TILE_SPACEISH,              w: TILE_NW_WALL|TILE_N_WALL}),

    Tile::new("degenerate-n", b"  ", Constraints{n: TILE_SPACE,     e: TILE_SPACE,  s: !TILE_SPACE, w: TILE_SPACE}),
    Tile::new("degenerate-e", b"  ", Constraints{n: TILE_SPACE,     e: TILE_SPACE,  s: TILE_SPACE,  w: !TILE_SPACE}),
    Tile::new("degenerate-s", b"  ", Constraints{n: !TILE_SPACE,    e: TILE_SPACE,  s: TILE_SPACE,  w: TILE_SPACE}),
    Tile::new("degenerate-w", b"  ", Constraints{n: TILE_SPACE,     e: !TILE_SPACE, s: TILE_SPACE,  w: TILE_SPACE}),

//    Tile::new("barrel-n", b"o^", Constraints{
//        n: TILE_FLOOR,
//        e: TILE_E_WALL|TILE_NE_WALL2|TILE_SE_WALL2|TILE_FLOORISH,
//        s: (TILE_BARREL|TILE_S_WALL) & !TILE_BARREL_S,
//        w: TILE_W_WALL|TILE_NW_WALL2|TILE_SW_WALL2|TILE_FLOORISH
//    }),
//    Tile::new("barrel-e", b"o>", Constraints{
//        n: TILE_N_WALL|TILE_NE_WALL2|TILE_NW_WALL2|TILE_FLOORISH,
//        e: TILE_FLOOR,
//        s: TILE_S_WALL|TILE_SE_WALL2|TILE_SW_WALL2|TILE_FLOORISH,
//        w: (TILE_BARREL|TILE_W_WALL) & !TILE_BARREL_W
//    }),
//    Tile::new("barrel-s", b"ov", Constraints{
//        n: (TILE_BARREL|TILE_N_WALL) & !TILE_BARREL_N,
//        e: TILE_E_WALL|TILE_NE_WALL2|TILE_SE_WALL2|TILE_FLOORISH,
//        s: TILE_FLOOR,
//        w: TILE_W_WALL|TILE_NW_WALL2|TILE_SW_WALL2|TILE_FLOORISH
//    }),
//    Tile::new("barrel-w", b"o<", Constraints{
//        n: TILE_N_WALL|TILE_NE_WALL2|TILE_NW_WALL2|TILE_FLOORISH,
//        e: (TILE_BARREL|TILE_E_WALL) & !TILE_BARREL_E,
//        s: TILE_S_WALL|TILE_SE_WALL2|TILE_SW_WALL2|TILE_FLOORISH,
//        w: TILE_FLOOR
//    }),
];

// a convenience mask for all tiles
pub const TILE_ALL: u64 = (1u64 << TILES.len()) - 1;
pub const TILE_NOTSPACE: u64 = TILE_ALL & !TILE_SPACE;
