#![allow(dead_code)]
// #![feature(box_syntax, box_patterns)]

mod context;
mod value;
mod ops;
mod interp;

use std::io::{Read, BufRead, stdin};
use std::env;
use std::fs::File;

use context::{Context};
use ops::{Ops};

fn main() {
    let mut ops = Ops::new();
    ops::core::init(&mut ops);
    ops::print::init(&mut ops);
    ops::stack::init(&mut ops);
    ops::control::init(&mut ops);
    ops::arith::init(&mut ops);
    ops::list::init(&mut ops);

    let mut ctx = Context::new();

    let mut args = env::args();
    args.next();
    match args.next() {
        Some(fname) => {
            let mut f = File::open(fname).unwrap();
            let mut code = String::new();
            f.read_to_string(&mut code).unwrap();
            let code = if code.starts_with("#!") {
                match code.find('\n') {
                    Some(l) => &code[(l+1)..],
                    None => ""
                }
                
            } else {
                &code
            };
            interp::interp(&mut ctx, &mut ops, code);
        },
        None => {
            let stdin = stdin();
            for line in stdin.lock().lines() {
                interp::interp(&mut ctx, &mut ops, &line.unwrap());
            }
        }
    }
}
