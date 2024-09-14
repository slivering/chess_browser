
#![feature(step_trait)] // FIXME: change this
#![allow(dead_code)]

#[macro_use]
mod macros;
mod generate;
mod units;
mod bit;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=generate/attack_tables.rs");
    println!("cargo:rerun-if-changed=generate/attack_gen.rs");
    println!("cargo:rerun-if-changed=generate/zobrist_gen.rs");
    println!("cargo:rerun-if-changed=generate/mod.rs");
    println!("cargo:rerun-if-changed=generate/zobrist_tables.rs");

    generate::all_tables();
}