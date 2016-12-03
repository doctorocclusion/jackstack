use context::Context;
use super::{Ops, Op};

impl_op!(OpRetop, c, {
	let v = c.pop().unwrap();
	c.push(v);
});

impl_op!(OpDel, c, {
	c.pop().unwrap();
});


pub fn init(ops: &mut Ops) {
	ops.add(String::from("retop"), OpRetop::new);
	ops.add(String::from("del"), OpDel::new);
}