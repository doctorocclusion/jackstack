use std::collections::{HashMap};
use context::{Context};

pub trait Op {
	fn apply(&self, &mut Context);
}

macro_rules! impl_op {
    ($n:ident, $c:ident, $x:expr) => {
    	struct $n;

    	impl $n {
    		fn new() -> Box<Op> {
    			Box::new($n {})
    		}
    	}

    	impl Op for $n {
    		fn apply(&self, $c: &mut Context) {$x}
    	}
    };
}

pub mod core;
pub mod print;
pub mod stack;

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