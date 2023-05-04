

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
pub const TILE_LR_E_TEE_B2      : u128 = 1 << 39;

pub const TILE_LL_E_INWALL_A    : u128 = 1 << 40;
pub const TILE_LL_E_DOOR        : u128 = 1 << 41;
pub const TILE_LL_E_INWALL_B    : u128 = 1 << 42;
pub const TILE_LL_E_JOINT_A     : u128 = 1 << 43;
pub const TILE_LL_E_JOINT_B     : u128 = 1 << 44;
pub const TILE_LL_E_TEE_A       : u128 = 1 << 45;
pub const TILE_LL_E_TEE_B       : u128 = 1 << 46;
pub const TILE_LL_E_TEE_B2      : u128 = 1 << 47;

pub const TILE_LL_S_INWALL_A    : u128 = 1 << 48;
pub const TILE_LL_S_DOOR        : u128 = 1 << 49;
pub const TILE_LL_S_INWALL_B    : u128 = 1 << 50;
pub const TILE_LL_S_TEE_A       : u128 = 1 << 51;
pub const TILE_LL_S_TEE_B       : u128 = 1 << 52;
pub const TILE_LL_S_TEE_B2      : u128 = 1 << 53;

pub const TILE_UL_S_INWALL_A    : u128 = 1 << 54;
pub const TILE_UL_S_DOOR        : u128 = 1 << 55;
pub const TILE_UL_S_INWALL_B    : u128 = 1 << 56;
pub const TILE_UL_S_JOINT_A     : u128 = 1 << 57;
pub const TILE_UL_S_JOINT_B     : u128 = 1 << 58;
pub const TILE_UL_S_TEE_A       : u128 = 1 << 59;
pub const TILE_UL_S_TEE_B       : u128 = 1 << 60;
pub const TILE_UL_S_TEE_B2      : u128 = 1 << 61;

pub const TILE_UL_W_INWALL_A    : u128 = 1 << 62;
pub const TILE_UL_W_DOOR        : u128 = 1 << 63;
pub const TILE_UL_W_INWALL_B    : u128 = 1 << 64;
pub const TILE_UL_W_TEE_A       : u128 = 1 << 65;
pub const TILE_UL_W_TEE_B       : u128 = 1 << 66;
pub const TILE_UL_W_TEE_B2      : u128 = 1 << 67;

pub const TILE_UR_W_INWALL_A    : u128 = 1 << 68;
pub const TILE_UR_W_DOOR        : u128 = 1 << 69;
pub const TILE_UR_W_INWALL_B    : u128 = 1 << 70;
pub const TILE_UR_W_JOINT_A     : u128 = 1 << 71;
pub const TILE_UR_W_JOINT_B     : u128 = 1 << 72;
pub const TILE_UR_W_TEE_A       : u128 = 1 << 73;
pub const TILE_UR_W_TEE_B       : u128 = 1 << 74;
pub const TILE_UR_W_TEE_B2      : u128 = 1 << 75;

pub const TILE_UR_N_INWALL_A    : u128 = 1 << 76;
pub const TILE_UR_N_DOOR        : u128 = 1 << 77;
pub const TILE_UR_N_INWALL_B    : u128 = 1 << 78;
pub const TILE_UR_N_TEE_A       : u128 = 1 << 79;
pub const TILE_UR_N_TEE_B       : u128 = 1 << 80;
pub const TILE_UR_N_TEE_B2      : u128 = 1 << 81;

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
//
// I sure do love changings this array size everytime I tweak anything here
//
pub const TILES: [Tile; 82] = [
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
    Tile::new("lr-n-inwall-a", b"| ", Constraints{
        n: TILE_LR_N_INWALL_A | TILE_LR_N_DOOR | TILE_LR_N_JOINT_A | TILE_LR_N_TEE_A | TILE_LR_E_TEE_B,
        e: TILE_FLOORISH | TILE_WALLISH,
        s: TILE_WALLISH | TILE_LR_N_INWALL_A | TILE_LR_N_TEE_A | TILE_LR_N_TEE_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("lr-n-door", b"  ", Constraints{
        n: TILE_LR_N_INWALL_B,
        e: TILE_FLOORISH,
        s: TILE_LR_N_INWALL_A,
        w: TILE_FLOORISH,
    }),
    Tile::new("lr-n-inwall-b", b"| ", Constraints{
        n: TILE_WALLISH | TILE_LR_N_INWALL_B | TILE_LR_N_JOINT_B | TILE_LR_N_TEE_B | TILE_LR_N_TEE_B2 | TILE_LR_E_TEE_A | TILE_LR_E_TEE_B2,
        e: TILE_FLOORISH | TILE_WALLISH,
        s: TILE_LR_N_INWALL_B | TILE_LR_N_DOOR | TILE_LR_N_TEE_B2,
        w: TILE_FLOORISH,
    }),
    Tile::new("lr-n-joint-a", b".-", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_A,
        w: TILE_FLOORISH,
    }),
    Tile::new("lr-n-joint-b", b".-", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_B,
        s: TILE_LR_N_INWALL_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("lr-n-tee-a", b"+-", Constraints{
        n: TILE_LR_N_INWALL_A,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_A,
        w: TILE_FLOORISH,
    }),
    Tile::new("lr-n-tee-b", b"+-", Constraints{
        n: TILE_LR_N_INWALL_A,
        e: TILE_LR_E_INWALL_B,
        s: TILE_LR_N_INWALL_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("lr-n-tee-b2", b"+-", Constraints{
        n: TILE_LR_N_INWALL_B,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_B,
        w: TILE_FLOORISH,
    }),

    Tile::new("lr-e-inwall-a", b"--", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_A | TILE_LR_E_DOOR | TILE_LR_E_TEE_A,
        s: TILE_FLOORISH | TILE_WALLISH,
        w: TILE_WALLISH | TILE_LR_E_INWALL_A | TILE_LR_N_JOINT_A | TILE_LR_N_TEE_A | TILE_LR_N_TEE_B2 | TILE_LR_E_TEE_A | TILE_LR_E_TEE_B,
    }),
    Tile::new("lr-e-door", b"  ", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_B,
        s: TILE_FLOORISH,
        w: TILE_LR_E_INWALL_A,
    }),
    Tile::new("lr-e-inwall-b", b"--", Constraints{
        n: TILE_FLOORISH,
        e: TILE_WALLISH | TILE_LR_E_INWALL_B | TILE_LR_N_JOINT_B | TILE_LR_E_TEE_B | TILE_LR_E_TEE_B2,
        s: TILE_FLOORISH | TILE_WALLISH,
        w: TILE_LR_E_INWALL_B | TILE_LR_E_DOOR | TILE_LR_N_JOINT_B | TILE_LR_E_TEE_B2,
    }),
    Tile::new("lr-e-tee-a", b"+-", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_B,
        w: TILE_LR_E_INWALL_A,
    }),
    Tile::new("lr-e-tee-b", b"+-", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_A,
        s: TILE_LR_N_INWALL_A,
        w: TILE_LR_E_INWALL_B,
    }),
    Tile::new("lr-e-tee-b2", b"+-", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LR_E_INWALL_B,
        s: TILE_LR_N_INWALL_B,
        w: TILE_LR_E_INWALL_B,
    }),

    //
    Tile::new("ll-e-inwall-a", b"--", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LL_E_INWALL_A | TILE_LL_E_DOOR | TILE_LL_E_JOINT_A | TILE_LL_E_TEE_A | TILE_LL_S_TEE_B,
        s: TILE_FLOORISH | TILE_WALLISH,
        w: TILE_WALLISH | TILE_LL_E_INWALL_A | TILE_LL_E_TEE_A | TILE_LL_E_TEE_B,
    }),
    Tile::new("ll-e-door", b"  ", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LL_E_INWALL_B,
        s: TILE_FLOORISH,
        w: TILE_LL_E_INWALL_A,
    }),
    Tile::new("ll-e-inwall-b", b"--", Constraints{
        n: TILE_FLOORISH,
        e: TILE_WALLISH | TILE_LL_E_INWALL_B | TILE_LL_E_JOINT_B | TILE_LL_E_TEE_B | TILE_LL_E_TEE_B2 | TILE_LL_S_TEE_A | TILE_LL_S_TEE_B2,
        s: TILE_FLOORISH | TILE_WALLISH,
        w: TILE_LL_E_INWALL_B | TILE_LL_E_DOOR | TILE_LL_E_TEE_B2,
    }),
    Tile::new("ll-e-joint-a", b"-.", Constraints{
        n: TILE_FLOORISH,
        e: TILE_FLOORISH,
        s: TILE_LL_S_INWALL_A,
        w: TILE_LL_E_INWALL_A,
    }),
    Tile::new("ll-e-joint-b", b"-.", Constraints{
        n: TILE_FLOORISH,
        e: TILE_FLOORISH,
        s: TILE_LL_S_INWALL_B,
        w: TILE_LL_E_INWALL_B,
    }),
    Tile::new("ll-e-tee-a", b"-+", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LL_E_INWALL_A,
        s: TILE_LL_S_INWALL_A,
        w: TILE_LL_E_INWALL_A,
    }),
    Tile::new("ll-e-tee-b", b"-+", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LL_E_INWALL_A,
        s: TILE_LL_S_INWALL_B,
        w: TILE_LL_E_INWALL_B,
    }),
    Tile::new("ll-e-tee-b2", b"-+", Constraints{
        n: TILE_FLOORISH,
        e: TILE_LL_E_INWALL_B,
        s: TILE_LL_S_INWALL_A,
        w: TILE_LL_E_INWALL_B,
    }),

    Tile::new("ll-s-inwall-a", b" |", Constraints{
        n: TILE_WALLISH | TILE_LL_S_INWALL_A | TILE_LL_E_JOINT_A | TILE_LL_E_TEE_A | TILE_LL_E_TEE_B2 | TILE_LL_S_TEE_A | TILE_LL_S_TEE_B,
        e: TILE_FLOORISH,
        s: TILE_LL_S_INWALL_A | TILE_LL_S_DOOR | TILE_LL_S_TEE_A,
        w: TILE_FLOORISH | TILE_WALLISH,
    }),
    Tile::new("ll-s-door", b"  ", Constraints{
        n: TILE_LL_S_INWALL_A,
        e: TILE_FLOORISH,
        s: TILE_LL_S_INWALL_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("ll-s-inwall-b", b" |", Constraints{
        n: TILE_LL_S_INWALL_B | TILE_LL_S_DOOR | TILE_LL_E_JOINT_B | TILE_LL_S_TEE_B2,
        e: TILE_FLOORISH,
        s: TILE_WALLISH | TILE_LL_S_INWALL_B | TILE_LL_E_JOINT_B | TILE_LL_S_TEE_B | TILE_LL_S_TEE_B2,
        w: TILE_FLOORISH | TILE_WALLISH,
    }),
    Tile::new("ll-s-tee-a", b"-+", Constraints{
        n: TILE_LL_S_INWALL_A,
        e: TILE_FLOORISH,
        s: TILE_LL_S_INWALL_A,
        w: TILE_LL_E_INWALL_B,
    }),
    Tile::new("ll-s-tee-b", b"-+", Constraints{
        n: TILE_LL_S_INWALL_B,
        e: TILE_FLOORISH,
        s: TILE_LL_S_INWALL_A,
        w: TILE_LL_E_INWALL_A,
    }),
    Tile::new("ll-s-tee-b2", b"-+", Constraints{
        n: TILE_LL_S_INWALL_B,
        e: TILE_FLOORISH,
        s: TILE_LL_S_INWALL_B,
        w: TILE_LL_E_INWALL_B,
    }),

    //
    Tile::new("ul-s-inwall-a", b" |", Constraints{
        n: TILE_WALLISH | TILE_UL_S_INWALL_A | TILE_UL_S_TEE_A | TILE_UL_S_TEE_B,
        e: TILE_FLOORISH,
        s: TILE_UL_S_INWALL_A | TILE_UL_S_DOOR | TILE_UL_S_JOINT_A | TILE_UL_S_TEE_A | TILE_UL_W_TEE_B,
        w: TILE_FLOORISH | TILE_WALLISH,
    }),
    Tile::new("ul-s-door", b"  ", Constraints{
        n: TILE_UL_S_INWALL_A,
        e: TILE_FLOORISH,
        s: TILE_UL_S_INWALL_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("ul-s-inwall-b", b" |", Constraints{
        n: TILE_UL_S_INWALL_B | TILE_UL_S_DOOR | TILE_UL_S_TEE_B2,
        e: TILE_FLOORISH,
        s: TILE_WALLISH | TILE_UL_S_INWALL_B | TILE_UL_S_JOINT_B | TILE_UL_S_TEE_B | TILE_UL_S_TEE_B2 | TILE_UL_W_TEE_A | TILE_UL_W_TEE_B2,
        w: TILE_FLOORISH | TILE_WALLISH,
    }),
    Tile::new("ul-s-joint-a", b"-'", Constraints{
        n: TILE_UL_S_INWALL_A,
        e: TILE_FLOORISH,
        s: TILE_FLOORISH,
        w: TILE_UL_W_INWALL_A,
    }),
    Tile::new("ul-s-joint-b", b"-'", Constraints{
        n: TILE_UL_S_INWALL_B,
        e: TILE_FLOORISH,
        s: TILE_FLOORISH,
        w: TILE_UL_W_INWALL_B,
    }),
    Tile::new("ul-s-tee-a", b"-+", Constraints{
        n: TILE_UL_S_INWALL_A,
        e: TILE_FLOORISH,
        s: TILE_UL_S_INWALL_A,
        w: TILE_UL_W_INWALL_A,
    }),
    Tile::new("ul-s-tee-b", b"-+", Constraints{
        n: TILE_UL_S_INWALL_B,
        e: TILE_FLOORISH,
        s: TILE_UL_S_INWALL_A,
        w: TILE_UL_W_INWALL_B,
    }),
    Tile::new("ul-s-tee-b2", b"-+", Constraints{
        n: TILE_UL_S_INWALL_B,
        e: TILE_FLOORISH,
        s: TILE_UL_S_INWALL_B,
        w: TILE_UL_W_INWALL_A,
    }),

    Tile::new("ul-w-inwall-a", b"--", Constraints{
        n: TILE_FLOORISH | TILE_WALLISH,
        e: TILE_WALLISH | TILE_UL_W_INWALL_A | TILE_UL_S_JOINT_A | TILE_UL_S_TEE_A | TILE_UL_S_TEE_B2 | TILE_UL_W_TEE_A | TILE_UL_W_TEE_B,
        s: TILE_FLOORISH,
        w: TILE_UL_W_INWALL_A | TILE_UL_W_DOOR | TILE_UL_W_TEE_A,
    }),
    Tile::new("ul-w-door", b"  ", Constraints{
        n: TILE_FLOORISH,
        e: TILE_UL_W_INWALL_A,
        s: TILE_FLOORISH,
        w: TILE_UL_W_INWALL_B,
    }),
    Tile::new("ul-w-inwall-b", b"--", Constraints{
        n: TILE_FLOORISH | TILE_WALLISH,
        e: TILE_UL_W_INWALL_B | TILE_UL_W_DOOR | TILE_UL_S_JOINT_B | TILE_UL_W_TEE_B2,
        s: TILE_FLOORISH,
        w: TILE_WALLISH | TILE_UL_W_INWALL_B | TILE_UL_S_JOINT_B | TILE_UL_W_TEE_B | TILE_UL_W_TEE_B2,
    }),
    Tile::new("ul-w-tee-a", b"-+", Constraints{
        n: TILE_UL_S_INWALL_B,
        e: TILE_UL_W_INWALL_A,
        s: TILE_FLOORISH,
        w: TILE_UL_W_INWALL_A,
    }),
    Tile::new("ul-w-tee-b", b"-+", Constraints{
        n: TILE_UL_S_INWALL_A,
        e: TILE_UL_W_INWALL_B,
        s: TILE_FLOORISH,
        w: TILE_UL_W_INWALL_A,
    }),
    Tile::new("ul-w-tee-b2", b"-+", Constraints{
        n: TILE_UL_S_INWALL_B,
        e: TILE_UL_W_INWALL_B,
        s: TILE_FLOORISH,
        w: TILE_UL_W_INWALL_B,
    }),

    //
    Tile::new("ur-w-inwall-a", b"--", Constraints{
        n: TILE_FLOORISH | TILE_WALLISH,
        e: TILE_WALLISH | TILE_UR_W_INWALL_A | TILE_UR_W_TEE_A | TILE_UR_W_TEE_B,
        s: TILE_FLOORISH,
        w: TILE_UR_W_INWALL_A | TILE_UR_W_DOOR | TILE_UR_W_JOINT_A | TILE_UR_W_TEE_A | TILE_UR_N_TEE_B,
    }),
    Tile::new("ur-w-door", b"  ", Constraints{
        n: TILE_FLOORISH,
        e: TILE_UR_W_INWALL_A,
        s: TILE_FLOORISH,
        w: TILE_UR_W_INWALL_B,
    }),
    Tile::new("ur-w-inwall-b", b"--", Constraints{
        n: TILE_FLOORISH | TILE_WALLISH,
        e: TILE_UR_W_INWALL_B | TILE_UR_W_DOOR | TILE_UR_W_TEE_B2,
        s: TILE_FLOORISH,
        w: TILE_WALLISH | TILE_UR_W_INWALL_B | TILE_UR_W_JOINT_B | TILE_UR_W_TEE_B | TILE_UR_W_TEE_B2 | TILE_UR_N_TEE_A | TILE_UR_N_TEE_B2,
    }),
    Tile::new("ur-w-joint-a", b"'-", Constraints{
        n: TILE_UR_N_INWALL_A,
        e: TILE_UR_W_INWALL_A,
        s: TILE_FLOORISH,
        w: TILE_FLOORISH,
    }),
    Tile::new("ur-w-joint-b", b"'-", Constraints{
        n: TILE_UR_N_INWALL_B,
        e: TILE_UR_W_INWALL_B,
        s: TILE_FLOORISH,
        w: TILE_FLOORISH,
    }),
    Tile::new("ur-w-tee-a", b"+-", Constraints{
        n: TILE_UR_N_INWALL_A,
        e: TILE_UR_W_INWALL_A,
        s: TILE_FLOORISH,
        w: TILE_UR_W_INWALL_A,
    }),
    Tile::new("ur-w-tee-b", b"+-", Constraints{
        n: TILE_UR_N_INWALL_B,
        e: TILE_UR_W_INWALL_B,
        s: TILE_FLOORISH,
        w: TILE_UR_W_INWALL_A,
    }),
    Tile::new("ur-w-tee-b2", b"+-", Constraints{
        n: TILE_UR_N_INWALL_A,
        e: TILE_UR_W_INWALL_B,
        s: TILE_FLOORISH,
        w: TILE_UR_W_INWALL_B,
    }),

    Tile::new("ur-n-inwall-a", b"| ", Constraints{
        n: TILE_UR_N_INWALL_A | TILE_UR_N_DOOR | TILE_UR_N_TEE_A,
        e: TILE_FLOORISH | TILE_WALLISH,
        s: TILE_WALLISH | TILE_UR_N_INWALL_A | TILE_UR_W_JOINT_A | TILE_UR_W_TEE_A | TILE_UR_W_TEE_B2 | TILE_UR_N_TEE_A | TILE_UR_N_TEE_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("ur-n-door", b"  ", Constraints{
        n: TILE_UR_N_INWALL_B,
        e: TILE_FLOORISH,
        s: TILE_UR_N_INWALL_A,
        w: TILE_FLOORISH,
    }),
    Tile::new("ur-n-inwall-b", b"| ", Constraints{
        n: TILE_WALLISH | TILE_UR_N_INWALL_B | TILE_UR_W_JOINT_B | TILE_UR_N_TEE_B | TILE_UR_N_TEE_B2,
        e: TILE_FLOORISH | TILE_WALLISH,
        s: TILE_UR_N_INWALL_B | TILE_UR_N_DOOR | TILE_UR_W_JOINT_B | TILE_UR_N_TEE_B2,
        w: TILE_FLOORISH,
    }),
    Tile::new("ur-n-tee-a", b"+-", Constraints{
        n: TILE_UR_N_INWALL_A,
        e: TILE_UR_W_INWALL_B,
        s: TILE_UR_N_INWALL_A,
        w: TILE_FLOORISH,
    }),
    Tile::new("ur-n-tee-b", b"+-", Constraints{
        n: TILE_UR_N_INWALL_A,
        e: TILE_UR_W_INWALL_A,
        s: TILE_UR_N_INWALL_B,
        w: TILE_FLOORISH,
    }),
    Tile::new("ur-n-tee-b2", b"+-", Constraints{
        n: TILE_UR_N_INWALL_B,
        e: TILE_UR_W_INWALL_B,
        s: TILE_UR_N_INWALL_B,
        w: TILE_FLOORISH,
    }),
];

// a convenience mask for all tiles
pub const TILE_ALL: u128 = (1u128 << TILES.len()) - 1;
