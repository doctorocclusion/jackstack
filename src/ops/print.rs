use context::Context;
use super::{Ops, Op};

impl_op!(OpPrint, c, println!("{}", c.print_stack()));
impl_op!(OpPeek, c, println!("{}", c.peek().unwrap().to_string()));
impl_op!(OpPop, c, println!("{}", c.pop().unwrap().to_string()));

pub fn init(ops: &mut Ops) {
	ops.add(String::from("print"), OpPrint::new);
	ops.add(String::from("peek"), OpPeek::new);
	ops.add(String::from("pop"), OpPop::new);
}