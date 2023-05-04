

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

pub const TILE_DEGENERATE_N : u64 = 1 << 1;
pub const TILE_DEGENERATE_E : u64 = 1 << 2;
pub const TILE_DEGENERATE_S : u64 = 1 << 3;
pub const TILE_DEGENERATE_W : u64 = 1 << 4;

pub const TILE_FLOOR        : u64 = 1 << 5;
pub const TILE_N_FLOOR2     : u64 = 1 << 6;
pub const TILE_E_FLOOR2     : u64 = 1 << 7;
pub const TILE_S_FLOOR2     : u64 = 1 << 8;
pub const TILE_W_FLOOR2     : u64 = 1 << 9;

pub const TILE_N_WALL       : u64 = 1 << 10;
pub const TILE_E_WALL       : u64 = 1 << 11;
pub const TILE_S_WALL       : u64 = 1 << 12;
pub const TILE_W_WALL       : u64 = 1 << 13;

pub const TILE_NE_WALL      : u64 = 1 << 14;
pub const TILE_SE_WALL      : u64 = 1 << 15;
pub const TILE_SW_WALL      : u64 = 1 << 16;
pub const TILE_NW_WALL      : u64 = 1 << 17;

pub const TILE_NE_WALL2     : u64 = 1 << 18;
pub const TILE_SE_WALL2     : u64 = 1 << 19;
pub const TILE_SW_WALL2     : u64 = 1 << 20;
pub const TILE_NW_WALL2     : u64 = 1 << 21;

pub const TILE_NE_WALL3     : u64 = 1 << 22;
pub const TILE_SE_WALL3     : u64 = 1 << 23;
pub const TILE_SW_WALL3     : u64 = 1 << 24;
pub const TILE_NW_WALL3     : u64 = 1 << 25;

pub const TILE_NE_WALL4     : u64 = 1 << 26;
pub const TILE_SE_WALL4     : u64 = 1 << 27;
pub const TILE_SW_WALL4     : u64 = 1 << 28;
pub const TILE_NW_WALL4     : u64 = 1 << 29;

pub const TILE_N_DOORA      : u64 = 1 << 30;
pub const TILE_E_DOORA      : u64 = 1 << 31;
pub const TILE_N_DOORB      : u64 = 1 << 32;
pub const TILE_E_DOORB      : u64 = 1 << 33;

pub const TILE_N_INWALLA    : u64 = 1 << 34;
pub const TILE_E_INWALLA    : u64 = 1 << 35;

pub const TILE_NE_JOINTA    : u64 = 1 << 36;
pub const TILE_SE_JOINTA    : u64 = 1 << 37;
pub const TILE_SW_JOINTA    : u64 = 1 << 38;
pub const TILE_NW_JOINTA    : u64 = 1 << 39;

pub const TILE_N_INWALLB    : u64 = 1 << 40;
pub const TILE_E_INWALLB    : u64 = 1 << 41;

pub const TILE_NE_JOINTB    : u64 = 1 << 42;
pub const TILE_SE_JOINTB    : u64 = 1 << 43;
pub const TILE_SW_JOINTB    : u64 = 1 << 44;
pub const TILE_NW_JOINTB    : u64 = 1 << 45;

// some common constraints
pub const TILE_SPACEISH: u64
    = TILE_SPACE
    | TILE_DEGENERATE_N
    | TILE_DEGENERATE_E
    | TILE_DEGENERATE_S
    | TILE_DEGENERATE_W;

pub const TILE_FLOORISH: u64
    = TILE_FLOOR
    | TILE_N_FLOOR2
    | TILE_E_FLOOR2
    | TILE_S_FLOOR2
    | TILE_W_FLOOR2
    | TILE_N_DOORA
    | TILE_E_DOORA
    | TILE_N_DOORB
    | TILE_E_DOORB;

pub const TILE_JOINTISHA: u64
    = TILE_NE_JOINTA
    | TILE_SE_JOINTA
    | TILE_SW_JOINTA
    | TILE_NW_JOINTA;

pub const TILE_JOINTISHB: u64
    = TILE_NE_JOINTB
    | TILE_SE_JOINTB
    | TILE_SW_JOINTB
    | TILE_NW_JOINTB;

pub const TILE_WALLISH: u64
    = TILE_N_WALL
    | TILE_E_WALL
    | TILE_S_WALL
    | TILE_W_WALL
    | TILE_NE_WALL
    | TILE_SE_WALL
    | TILE_SW_WALL
    | TILE_NW_WALL
    | TILE_NE_WALL2
    | TILE_SE_WALL2
    | TILE_SW_WALL2
    | TILE_NW_WALL2
    | TILE_NE_WALL3
    | TILE_SE_WALL3
    | TILE_SW_WALL3
    | TILE_NW_WALL3
    | TILE_NE_WALL4
    | TILE_SE_WALL4
    | TILE_SW_WALL4
    | TILE_NW_WALL4;

// all tiles in our system
pub const TILES: [Tile; 46] = [
    // space
    Tile::new("space", b"  ", Constraints::ALL),

    Tile::new("degenerate-n", b"  ", Constraints{
        n: TILE_SPACE,
        e: TILE_SPACE,
        s: !TILE_SPACE,
        w: TILE_SPACE,
    }),
    Tile::new("degenerate-e", b"  ", Constraints{
        n: TILE_SPACE,
        e: TILE_SPACE,
        s: TILE_SPACE,
        w: !TILE_SPACE,
    }),
    Tile::new("degenerate-s", b"  ", Constraints{
        n: !TILE_SPACE,
        e: TILE_SPACE,
        s: TILE_SPACE,
        w: TILE_SPACE,
    }),
    Tile::new("degenerate-w", b"  ", Constraints{
        n: TILE_SPACE,
        e: !TILE_SPACE,
        s: TILE_SPACE,
        w: TILE_SPACE,
    }),

    // inside things
    Tile::new("floor", b"  ", Constraints{
        n: !TILE_SPACEISH,
        e: !TILE_SPACEISH,
        s: !TILE_SPACEISH,
        w: !TILE_SPACEISH,
    }),
    Tile::new("n-floor2", b"  ", Constraints{
        n: !TILE_SPACEISH,
        e: !TILE_SPACEISH,
        s: TILE_FLOORISH & !TILE_S_FLOOR2,
        w: !TILE_SPACEISH,
    }),
    Tile::new("e-floor2", b"  ", Constraints{
        n: !TILE_SPACEISH,
        e: !TILE_SPACEISH,
        s: !TILE_SPACEISH,
        w: TILE_FLOORISH & !TILE_W_FLOOR2,
    }),
    Tile::new("s-floor2", b"  ", Constraints{
        n: TILE_FLOORISH & !TILE_N_FLOOR2,
        e: !TILE_SPACEISH,
        s: !TILE_SPACEISH,
        w: !TILE_SPACEISH,
    }),
    Tile::new("w-floor2", b"  ", Constraints{
        n: !TILE_SPACEISH,
        e: TILE_FLOORISH & !TILE_E_FLOOR2,
        s: !TILE_SPACEISH,
        w: !TILE_SPACEISH,
    }),

    // walls
    Tile::new("n-wall", b"--", Constraints{
        n: TILE_SPACEISH,
        e: TILE_WALLISH,
        s: !TILE_SPACEISH,
        w: TILE_WALLISH,
    }),
    Tile::new("e-wall", b"| ", Constraints{
        n: !TILE_SPACEISH,
        e: TILE_SPACEISH,
        s: !TILE_SPACEISH,
        w: !TILE_SPACEISH,
    }),
    Tile::new("s-wall", b"--", Constraints{
        n: !TILE_SPACEISH,
        e: TILE_WALLISH,
        s: TILE_SPACEISH,
        w: TILE_WALLISH,
    }),
    Tile::new("w-wall", b" |", Constraints{
        n: TILE_WALLISH,
        e: !TILE_SPACEISH,
        s: TILE_WALLISH,
        w: TILE_SPACEISH,
    }),

    Tile::new("ne-wall", b". ", Constraints{
        n: TILE_SPACEISH,
        e: TILE_SPACEISH,
        s: TILE_WALLISH,
        w: TILE_WALLISH,
    }),
    Tile::new("se-wall", b"' ", Constraints{
        n: TILE_WALLISH,
        e: TILE_SPACEISH,
        s: TILE_SPACEISH,
        w: TILE_WALLISH,
    }),
    Tile::new("sw-wall", b" '", Constraints{
        n: TILE_WALLISH,
        e: TILE_WALLISH,
        s: TILE_SPACEISH,
        w: TILE_SPACEISH,
    }),
    Tile::new("nw-wall", b" .", Constraints{
        n: TILE_SPACEISH,
        e: TILE_WALLISH,
        s: TILE_WALLISH,
        w: TILE_SPACEISH,
    }),

    Tile::new("ne-wall2", b" '", Constraints{
        n: TILE_NE_WALL,
        e: TILE_NE_WALL,
        s: !TILE_SPACEISH,
        w: !TILE_SPACEISH,
    }),
    Tile::new("se-wall2", b" .", Constraints{
        n: !TILE_SPACEISH,
        e: TILE_SE_WALL,
        s: TILE_SE_WALL,
        w: !TILE_SPACEISH,
    }),
    Tile::new("sw-wall2", b". ", Constraints{
        n: !TILE_SPACEISH,
        e: !TILE_SPACEISH,
        s: TILE_SW_WALL,
        w: TILE_SW_WALL,
    }),
    Tile::new("nw-wall2", b"' ", Constraints{
        n: TILE_NW_WALL,
        e: !TILE_SPACEISH,
        s: !TILE_SPACEISH,
        w: TILE_NW_WALL,
    }),

    Tile::new("ne-wall3", b"+-", Constraints{
        n: TILE_E_WALL,
        e: TILE_NE_WALL|TILE_N_WALL,
        s: !TILE_SPACEISH,
        w: !TILE_SPACEISH,
    }),
    Tile::new("se-wall3", b"+-", Constraints{
        n: !TILE_SPACEISH,
        e: TILE_S_WALL,
        s: TILE_SE_WALL|TILE_E_WALL,
        w: !TILE_SPACEISH,
    }),
    Tile::new("sw-wall3", b"-+", Constraints{
        n: !TILE_SPACEISH,
        e: !TILE_SPACEISH,
        s: TILE_W_WALL,
        w: TILE_SW_WALL|TILE_S_WALL,
    }),
    Tile::new("nw-wall3", b"-+", Constraints{
        n: TILE_NW_WALL|TILE_W_WALL,
        e: !TILE_SPACEISH,
        s: !TILE_SPACEISH,
        w: TILE_N_WALL,
    }),

    Tile::new("ne-wall4", b"+-", Constraints{
        n: TILE_NE_WALL|TILE_E_WALL,
        e: TILE_N_WALL,
        s: !TILE_SPACEISH,
        w: !TILE_SPACEISH,
    }),
    Tile::new("se-wall4", b"+-", Constraints{
        n: !TILE_SPACEISH,
        e: TILE_SE_WALL|TILE_S_WALL,
        s: TILE_E_WALL,
        w: !TILE_SPACEISH,
    }),
    Tile::new("sw-wall4", b"-+", Constraints{
        n: !TILE_SPACEISH,
        e: !TILE_SPACEISH,
        s: TILE_SW_WALL|TILE_W_WALL,
        w: TILE_S_WALL,
    }),
    Tile::new("nw-wall4", b"-+", Constraints{
        n: TILE_W_WALL,
        e: !TILE_SPACEISH,
        s: !TILE_SPACEISH,
        w: TILE_NW_WALL|TILE_N_WALL,
    }),

    // inner wall things
    Tile::new("n-doora", b"  ", Constraints{
        n: TILE_FLOORISH,
        e: TILE_N_INWALLA,
        s: TILE_FLOORISH,
        w: TILE_N_INWALLB,
    }),
    Tile::new("e-doora", b"  ", Constraints{
        n: 0, //TILE_E_INWALLA,
        e: TILE_FLOORISH,
        s: TILE_E_INWALLB,
        w: TILE_FLOORISH,
    }),
    Tile::new("n-doorb", b"  ", Constraints{
        n: TILE_FLOORISH,
        e: 0, //TILE_N_INWALLB,
        s: TILE_FLOORISH,
        w: TILE_N_INWALLA,
    }),
    Tile::new("e-doorb", b"  ", Constraints{
        n: 0, //TILE_E_INWALLB,
        e: TILE_FLOORISH,
        s: TILE_E_INWALLA,
        w: TILE_FLOORISH,
    }),

    Tile::new("n-inwalla", b"==", Constraints{
        n: TILE_FLOORISH,
        e: TILE_WALLISH | TILE_N_INWALLA | TILE_N_DOORA | TILE_N_DOORB | TILE_JOINTISHA,
        s: TILE_FLOORISH,
        w: TILE_WALLISH | TILE_N_INWALLA | TILE_N_DOORA | TILE_N_DOORB | TILE_JOINTISHA,
    }),
    Tile::new("e-inwalla", b"||", Constraints{
        n: TILE_WALLISH | TILE_E_INWALLA | TILE_E_DOORA | TILE_E_DOORB | TILE_JOINTISHA,
        e: TILE_FLOORISH,
        s: TILE_WALLISH | TILE_E_INWALLA | TILE_E_DOORA | TILE_E_DOORB | TILE_JOINTISHA,
        w: TILE_FLOORISH,
    }),

    Tile::new("ne-jointa", b"++", Constraints{
        n: TILE_E_INWALLA,
        e: TILE_N_INWALLA,
        s: TILE_FLOORISH | TILE_E_INWALLA,
        w: TILE_FLOORISH | TILE_N_INWALLA,
    }),
    Tile::new("se-jointa", b"++", Constraints{
        n: TILE_FLOORISH | TILE_E_INWALLA,
        e: TILE_N_INWALLA,
        s: TILE_E_INWALLA,
        w: TILE_FLOORISH | TILE_N_INWALLA,
    }),
    Tile::new("sw-jointa", b"++", Constraints{
        n: TILE_FLOORISH | TILE_E_INWALLA,
        e: TILE_FLOORISH | TILE_N_INWALLA,
        s: TILE_E_INWALLA,
        w: TILE_N_INWALLA,
    }),
    Tile::new("nw-jointa", b"++", Constraints{
        n: TILE_E_INWALLA,
        e: TILE_FLOORISH | TILE_N_INWALLA,
        s: TILE_FLOORISH | TILE_E_INWALLA,
        w: TILE_N_INWALLA,
    }),

    Tile::new("n-inwallb", b"=b", Constraints{
        n: TILE_FLOORISH,
        e: TILE_WALLISH | TILE_N_INWALLB | TILE_N_DOORA | TILE_N_DOORB | TILE_JOINTISHB,
        s: TILE_FLOORISH,
        w: TILE_WALLISH | TILE_N_INWALLB | TILE_N_DOORA | TILE_N_DOORB | TILE_JOINTISHB,
    }),
    Tile::new("e-inwallb", b"|b", Constraints{
        n: TILE_WALLISH | TILE_E_INWALLB | TILE_E_DOORA | TILE_E_DOORB | TILE_JOINTISHB,
        e: TILE_FLOORISH,
        s: TILE_WALLISH | TILE_E_INWALLB | TILE_E_DOORA | TILE_E_DOORB | TILE_JOINTISHB,
        w: TILE_FLOORISH,
    }),

    Tile::new("ne-jointb", b"+b", Constraints{
        n: TILE_E_INWALLB,
        e: TILE_N_INWALLB,
        s: TILE_FLOORISH | TILE_E_INWALLB,
        w: TILE_FLOORISH | TILE_N_INWALLB,
    }),
    Tile::new("se-jointb", b"+b", Constraints{
        n: TILE_FLOORISH | TILE_E_INWALLB,
        e: TILE_N_INWALLB,
        s: TILE_E_INWALLB,
        w: TILE_FLOORISH | TILE_N_INWALLB,
    }),
    Tile::new("sw-jointb", b"+b", Constraints{
        n: TILE_FLOORISH | TILE_E_INWALLB,
        e: TILE_FLOORISH | TILE_N_INWALLB,
        s: TILE_E_INWALLB,
        w: TILE_N_INWALLB,
    }),
    Tile::new("nw-jointb", b"+b", Constraints{
        n: TILE_E_INWALLB,
        e: TILE_FLOORISH | TILE_N_INWALLB,
        s: TILE_FLOORISH | TILE_E_INWALLB,
        w: TILE_N_INWALLB,
    }),
];

// a convenience mask for all tiles
pub const TILE_ALL: u64 = (1u64 << TILES.len()) - 1;
