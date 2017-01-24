use std::collections::{HashMap};
use context::{Context};

pub trait Op {
    fn apply(&self, &mut Context);
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

macro_rules! impl_op {
    ($n:ident, $c:ident, $x:expr) => (
        struct $n;

        impl $n {
            fn new() -> Box<Op> {
                Box::new($n {})
            }
        }

        impl Op for $n {
            fn apply(&self, $c: &mut Context) {$x}
        }
    )
}

macro_rules! pattern_op {
    ($n:ident, $ctx:ident, [$pat:pat], [$(push:ident),*], $x:expr) => (impl_op!($n, $ctx, {
        let input;
        {
            let pop_count = 0usize $(+ replace_expr!($o 1usize))*;
            let mut drain = $ctx.drain(pop_count);
            intput = ($(*drain.next().unwrap()),*);
        }
        match intput {
            ($pat) => {
                $(
                    let $push;
                )*
                $x;
                $(
                    $ctx.push(Box::new($u));
                )*
            },
            _ => panic!()
        }
    });)
}

macro_rules! simple_op {
    ($n:ident, $ctx:ident, ($($o:ident: $ot:ident),*), ($($u:ident: $ut:ident),*), $x:expr) => (impl_op!($n, $ctx, {
        $(
            let $o;
        )*
        {
            let pop_count = 0usize $(+ replace_expr!($o 1usize))*;
            let mut drain = $ctx.drain(pop_count);
            $(
                $o = if let Value::$ot(x) = *drain.next().unwrap() {x} else { panic!() };
            )*
        }
        $(
            let $u;
        )*
        $x;
        $(
            $ctx.push(Box::new(Value::$ut($u)));
        )*
    });)
}

pub mod core;
pub mod print;
pub mod stack;
pub mod control;
pub mod arith;
pub mod list;

pub struct Ops {
    reg: HashMap<String, fn() -> Box<Op>>
}

impl Ops {
    pub fn new() -> Ops {
        Ops {
            reg: HashMap::new()
        }
    }

    pub fn get(&self, op: &str) -> Option<Box<Op>> {
        match self.reg.get(op) {
            Some(o) => Some(o()),
            _ => None
        }
    }

    pub fn add(&mut self, name: String, gen: fn() -> Box<Op>) {
        self.reg.insert(name, gen);
    }
}