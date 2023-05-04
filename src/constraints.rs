

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
    pub n: u128,
    pub e: u128,
    pub s: u128,
    pub w: u128,
}

impl Constraints {
    const NONE: Constraints = Constraints::same(0);
    const ALL:  Constraints = Constraints::same(u128::MAX);

    const fn same(c: u128) -> Constraints {
        Constraints{n: c, e: c, s: c, w: c}
    }

    pub fn dir(&self, dir: Dir) -> u128 {
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


// each tile gets a bit in a u128, this is also its index in the following array
//
// unfortunately Rust doesn't make this easy for us, and I'm too lazy to
// write a proc_macro
//
pub const TILE_SPACE            : u128 = 1 << 0;

pub const TILE_DEGENERATE_N     : u128 = 1 << 1;
pub const TILE_DEGENERATE_E     : u128 = 1 << 2;
pub const TILE_DEGENERATE_S     : u128 = 1 << 3;
pub const TILE_DEGENERATE_W     : u128 = 1 << 4;

pub const TILE_FLOOR            : u128 = 1 << 5;

pub const TILE_N_WALL           : u128 = 1 << 6;
pub const TILE_E_WALL           : u128 = 1 << 7;
pub const TILE_S_WALL           : u128 = 1 << 8;
pub const TILE_W_WALL           : u128 = 1 << 9;

pub const TILE_NE_WALL          : u128 = 1 << 10;
pub const TILE_SE_WALL          : u128 = 1 << 11;
pub const TILE_SW_WALL          : u128 = 1 << 12;
pub const TILE_NW_WALL          : u128 = 1 << 13;

pub const TILE_NE_WALL2         : u128 = 1 << 14;
pub const TILE_SE_WALL2         : u128 = 1 << 15;
pub const TILE_SW_WALL2         : u128 = 1 << 16;
pub const TILE_NW_WALL2         : u128 = 1 << 17;

pub const TILE_NE_WALL3         : u128 = 1 << 18;
pub const TILE_SE_WALL3         : u128 = 1 << 19;
pub const TILE_SW_WALL3         : u128 = 1 << 20;
pub const TILE_NW_WALL3         : u128 = 1 << 21;

pub const TILE_NE_WALL4         : u128 = 1 << 22;
pub const TILE_SE_WALL4         : u128 = 1 << 23;
pub const TILE_SW_WALL4         : u128 = 1 << 24;
pub const TILE_NW_WALL4         : u128 = 1 << 25;

pub const TILE_LR_N_INWALL_A    : u128 = 1 << 26;
pub const TILE_LR_N_DOOR        : u128 = 1 << 27;
pub const TILE_LR_N_INWALL_B    : u128 = 1 << 28;
pub const TILE_LR_N_JOINT_A     : u128 = 1 << 29;
pub const TILE_LR_N_JOINT_B     : u128 = 1 << 30;
pub const TILE_LR_N_TEE_A       : u128 = 1 << 31;
pub const TILE_LR_N_TEE_B       : u128 = 1 << 32;
pub const TILE_LR_N_TEE_B2      : u128 = 1 << 33;

pub const TILE_LR_E_INWALL_A    : u128 = 1 << 34;
pub const TILE_LR_E_DOOR        : u128 = 1 << 35;
pub const TILE_LR_E_INWALL_B    : u128 = 1 << 36;
pub const TILE_LR_E_TEE_A       : u128 = 1 << 37;
pub const TILE_LR_E_TEE_B       : u128 = 1 << 38;

// some common constraints
pub const TILE_SPACEISH: u128
    = TILE_SPACE
    | TILE_DEGENERATE_N
    | TILE_DEGENERATE_E
    | TILE_DEGENERATE_S
    | TILE_DEGENERATE_W;

pub const TILE_FLOORISH: u128
    = TILE_FLOOR;

pub const TILE_WALLISH: u128
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
pub const TILES: [Tile; 39] = [
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

    // outside walls
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

    // inside walls
    //
    // I handwrote this and regret everything
    //
    // note this forms a sort of fsm that preserves a "one door for each room"
    // rull via coloring
    //
    Tile::new("n-inwall-a", b"|a", Constraints{
        n: TILE_LR_N_INWALL_A | TILE_LR_N_DOOR | TILE_LR_N_JOINT_A | TILE_LR_N_TEE_A | TILE_LR_E_TEE_B,
        e: TILE_FLOORISH,
        s: TILE_WALLISH | TILE_LR_N_INWALL_A | TILE_LR_N_TEE_A | TILE_LR_N_TEE_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("n-door", b"  ", Constraints{
        n: TILE_LR_N_INWALL_B,
        e: TILE_FLOORISH,
        s: TILE_LR_N_INWALL_A,
        w: TILE_FLOORISH,
    }),
    Tile::new("n-inwall-b", b"|b", Constraints{
        n: TILE_WALLISH | TILE_LR_N_INWALL_B | TILE_LR_N_JOINT_B | TILE_LR_N_TEE_B | TILE_LR_N_TEE_B2 | TILE_LR_E_TEE_A,
        e: TILE_FLOORISH,
        s: TILE_LR_N_INWALL_B | TILE_LR_N_DOOR | TILE_LR_N_TEE_B2,
        w: TILE_FLOORISH,
    }),
    Tile::new("n-joint-a", b"+a", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_A,
        w: TILE_FLOORISH,
    }),
    Tile::new("n-joint-b", b"+b", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_B,
        s: TILE_LR_N_INWALL_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("n-tee-a", b"ta", Constraints{
        n: TILE_LR_N_INWALL_A,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_A,
        w: TILE_FLOORISH,
    }),
    Tile::new("n-tee-b", b"tb", Constraints{
        n: TILE_LR_N_INWALL_A,
        e: TILE_LR_E_INWALL_B,
        s: TILE_LR_N_INWALL_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("n-tee-b2", b"tB", Constraints{
        n: TILE_LR_N_INWALL_B,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_B,
        w: TILE_FLOORISH,
    }),

    Tile::new("e-inwall-a", b"-a", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_A | TILE_LR_E_DOOR | TILE_LR_E_TEE_A,
        s: TILE_FLOORISH,
        w: TILE_WALLISH | TILE_LR_E_INWALL_A | TILE_LR_N_JOINT_A | TILE_LR_N_TEE_A | TILE_LR_N_TEE_B2 | TILE_LR_E_TEE_A | TILE_LR_E_TEE_B,
    }),
    Tile::new("e-door", b"  ", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_B,
        s: TILE_FLOORISH,
        w: TILE_LR_E_INWALL_A,
    }),
    Tile::new("e-inwall-b", b"-b", Constraints{
        n: TILE_FLOORISH,
        e: TILE_WALLISH | TILE_LR_E_INWALL_B | TILE_LR_N_JOINT_B | TILE_LR_E_TEE_B,
        s: TILE_FLOORISH,
        w: TILE_LR_E_INWALL_B | TILE_LR_E_DOOR | TILE_LR_N_JOINT_B,
    }),
    Tile::new("e-tee-a", b"Tb", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_B,
        w: TILE_LR_E_INWALL_A,
    }),
    Tile::new("e-tee-b", b"TB", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_A,
        w: TILE_LR_E_INWALL_B,
    }),
];

// a convenience mask for all tiles
pub const TILE_ALL: u128 = (1u128 << TILES.len()) - 1;
