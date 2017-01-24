use context::Context;
use super::{Ops, Op};

impl_op!(OpRetop, c, {
    let v = c.pop().unwrap();
    c.push(v);
});

impl_op!(OpDel, c, {
    c.pop().unwrap();
});

impl_op!(OpDup, c, {
    let dup = c.peek().unwrap().clone();
    c.push(dup);
});

impl_op!(OpSwap, c, {
    let b = c.pop().unwrap();
    let a = c.pop().unwrap();
    c.push(b);
    c.push(a);
});

pub fn init(ops: &mut Ops) {
    ops.add(String::from("retop"), OpRetop::new);
    ops.add(String::from("del"), OpDel::new);
    ops.add(String::from("dup"), OpDup::new);
    ops.add(String::from("swap"), OpSwap::new);
}