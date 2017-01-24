use context::Context;
use super::{Ops, Op};
use value::Value;

impl_op!(OpListPush, ctx, {
    let v = ctx.pop().unwrap();
    let mut l = ctx.peek_mut().unwrap();
    match l.as_mut() {
        &mut Value::List(ref mut l) => l.push(v),
        _ => panic!()
    }
});

impl_op!(OpListPop, ctx, {
    let o;
    {
        let mut l = ctx.peek_mut().unwrap();
        match l.as_mut() {
            &mut Value::List(ref mut l) => o = l.pop().unwrap(),
            _ => panic!()
        }
    }
    ctx.push(o);
});

impl_op!(OpDelist, ctx, {
    let l = ctx.pop().unwrap();
    match *l {
       Value::List(mut l) => ctx.top_frame().stack.append(&mut l),
        _ => panic!()
    }
});

impl_op!(OpEnlist, ctx, {
    let n = ctx.pop().unwrap();
    let o;
    match *n {
       Value::Double(n) => {
            let mut v: Vec<Box<Value>> = ctx.drain(n as usize).collect();
            v.reverse();
            o = Box::new(Value::List(v))
        },
        _ => panic!()
    }
    ctx.push(o);
});

impl_op!(OpListReverse, ctx, {
    let mut l = ctx.peek_mut().unwrap();
    match l.as_mut() {
        &mut Value::List(ref mut l) => l.reverse(),
        _ => panic!()
    }
});

pub fn init(ops: &mut Ops) {
    ops.add(String::from("lpush"), OpListPush::new);
    ops.add(String::from("lpop"), OpListPop::new);
    ops.add(String::from("delist"), OpDelist::new);
    ops.add(String::from("enlist"), OpEnlist::new);
    ops.add(String::from("lrev"), OpListReverse::new);
}