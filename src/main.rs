extern crate jackstack_lib as jacklib;

use std::io::{Read, BufRead, stdin};
use std::env;
use std::fs::File;

use jacklib::{Ops, Context};

fn main() {
    let mut ops = Ops::new();
    jacklib::ops::core::init(&mut ops);
    jacklib::ops::print::init(&mut ops);
    jacklib::ops::stack::init(&mut ops);
    jacklib::ops::control::init(&mut ops);
    jacklib::ops::arith::init(&mut ops);
    jacklib::ops::list::init(&mut ops);

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
            jacklib::interp(&mut ctx, &mut ops, code);
        },
        None => {
            let stdin = stdin();
            for line in stdin.lock().lines() {
                jacklib::interp(&mut ctx, &mut ops, &line.unwrap());
            }
        }
    }
}