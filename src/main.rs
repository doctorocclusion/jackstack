#![allow(dead_code)]

mod context;
mod value;
mod ops;
mod interp;

use std::io::{BufRead, stdin};

use context::{Context};
use ops::{Ops};

fn main() {
    let mut ops = Ops::new();
    ops::core::init(&mut ops);
    ops::print::init(&mut ops);
    ops::stacked::init(&mut ops);

    let mut ctx = Context::new();

    let stdin = stdin();
    for line in stdin.lock().lines() {
       interp::interp(&mut ctx, &mut ops, line.unwrap());
    }
}
