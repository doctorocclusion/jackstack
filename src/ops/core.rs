use context::Context;
use super::{Ops, Op};

impl_op!(OpEnd, c, c.exit_frame());
impl_op!(OpEndList, c, c.exit_frame_list());
impl_op!(OpBegin, c, c.enter_frame());
impl_op!(OpComma, c, c.hide_frame());

pub fn init(ops: &mut Ops) {
	ops.add(String::from("("), OpBegin::new);
	ops.add(String::from("["), OpBegin::new);
	ops.add(String::from(")"), OpEnd::new);
	ops.add(String::from("]"), OpEndList::new);
	ops.add(String::from(","), OpComma::new);
}