use context::Context;
use super::{Ops, Op};
use value::Value;

simple_op!(OpAdd, (b: Double, a: Double), (o: Double), o = a + b);
simple_op!(OpSub, (b: Double, a: Double), (o: Double), o = a - b);
simple_op!(OpMul, (b: Double, a: Double), (o: Double), o = a * b);
simple_op!(OpDiv, (b: Double, a: Double), (o: Double), o = a / b);
simple_op!(OpSqrt, (a: Double), (o: Double), o = a.sqrt());
simple_op!(OpExp, (a: Double), (o: Double), o = a.exp());
simple_op!(OpPow, (b: Double, a: Double), (o: Double), o = a.powf(b));
simple_op!(OpSin, (a: Double), (o: Double), o = a.sin());
simple_op!(OpCos, (a: Double), (o: Double), o = a.cos());
simple_op!(OpTan, (a: Double), (o: Double), o = a.tan());

pub fn init(ops: &mut Ops) {
    ops.add(String::from("add"), OpAdd::new);
    ops.add(String::from("sub"), OpSub::new);
    ops.add(String::from("mul"), OpMul::new);
    ops.add(String::from("div"), OpDiv::new);
    ops.add(String::from("sqrt"), OpSqrt::new);
    ops.add(String::from("exp"), OpExp::new);
    ops.add(String::from("pow"), OpPow::new);
    ops.add(String::from("sin"), OpSin::new);
    ops.add(String::from("cos"), OpCos::new);
    ops.add(String::from("tan"), OpTan::new);
}