use std::fs;
use std::io::{Write, Result as IoResult};

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};


pub fn write_in(f: &mut fs::File) -> IoResult<()> {
    use crate::units::*;

    let mut rng = SmallRng::seed_from_u64(0xffbeefdec0de2020);
    let mut write_table = |f: &mut fs::File, n: usize| -> IoResult<()> {
        writeln!(f, "[")?;
        for _i in 0..n {
            writeln!(f, "    {:#x},", rng.next_u64())?;
        }
        writeln!(f, "];")?;
        Ok(())
    };

    writeln!(f, "pub const INITIAL_HASH: Hash = 0x123456789abcdef;")?;
    writeln!(f, "pub const NONE_HASH: Hash = 0xfedcba987654321;")?;

    write!(f, "const HASH_PIECE: [Hash; Square::NUM * NUM_PIECES] = ")?;
    write_table(f, NUM_PIECES * Square::NUM)?;
    write!(f, "const HASH_SQUARE: [Hash; Square::NUM] = ")?;
    write_table(f, Square::NUM)?;
    write!(f, "const HASH_COLOR: [Hash; NUM_PLAYERS] = ")?;
    write_table(f, NUM_PLAYERS)?;
    write!(f, "const HASH_RIGHTS: [Hash; 16] = ")?;
    write_table(f, 16)?;
    Ok(())
}