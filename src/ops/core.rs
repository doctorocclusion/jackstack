use context::Context;
use value::{Value};
use super::{Ops, Op};

impl_op!(OpEnd, c, c.exit_frame());
impl_op!(OpEndList, c, c.exit_frame_list());
impl_op!(OpBegin, c, c.enter_frame());
impl_op!(OpComma, c, c.hide_frame());
impl_op!(OpRun, c, {
    let v = c.pop().unwrap();
    match *v {
        Value::Lambda(f) => f(c),
        _ => panic!()
    }
});

pub fn init(ops: &mut Ops) {
    ops.add(String::from("("), OpBegin::new);
    ops.add(String::from("["), OpBegin::new);
    ops.add(String::from(")"), OpEnd::new);
    ops.add(String::from("]"), OpEndList::new);
    ops.add(String::from(","), OpComma::new);
    ops.add(String::from("run"), OpRun::new);
}