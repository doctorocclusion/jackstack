use context::Context;
use value::{Value};
use super::{Ops, Op};

impl_op!(OpRepeat, ctx, {
	let n;
	let f;
	{
		let mut dr = ctx.drain(2);
		n = *dr.next().unwrap();
		f = *dr.next().unwrap();
	}
	match (n, f) {
		(Value::Double(n), Value::Lambda(f)) => {
			for _ in 0..(n as usize) {
				f(ctx);
			}
		},
		_ => panic!()
	}
	
});

pub fn init(ops: &mut Ops) {
	ops.add(String::from("repeat"), OpRepeat::new);
}