use std::fs::File;
use std::path::Path;

mod zobrist_gen;
mod attack_gen;

pub fn all_tables() {
    let mut path = Path::new("./src/generate/zobrist_tables.rs");
    let mut f = File::create(path).expect("Could not create file: `zobrist_tables.rs`");
    zobrist_gen::write_in(&mut f).unwrap();

    path = Path::new("./src/generate/attack_tables.rs");
    f = File::create(path).expect("Could not create file: `attack_tables.rs`");
    attack_gen::write_in(&mut f).unwrap();
}