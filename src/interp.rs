use std::str::{Chars};
use std::iter::{Iterator};
use std::collections::{LinkedList};

use context::Context;
use value::{Value};
use ops::{Ops, Op};
use std::rc::{Rc};

struct TokenIter<'a> {
	str: Chars<'a>,
	queue: LinkedList<String>
}

impl<'a> TokenIter<'a> {
	fn new(str: &String) -> TokenIter {
		TokenIter {
			str: str.chars(),
			queue: LinkedList::new()
		}
	}

	fn newtok(&mut self, tok: String) -> bool {
		if tok.len() != 0 {
			self.queue.push_back(tok);
			true
		} else {
			false
		}
	}

	fn parse(&mut self) {
		let mut out = String::new();
		loop {
			match self.str.next() {
				Some(c) => match c {
					'['|']'|'{'|'}'|'('|')'|',' => {
						let flag = self.newtok(out);
						self.queue.push_back(c.to_string());
						if flag {return;} else {out = String::new();}
					},
					_ if c.is_whitespace() => {
						if self.newtok(out) { return; } else { out = String::new(); }
					},
					_ => out.push(c)
				},
				None => {
					self.newtok(out);
					return;
				}
			}
		}
	}
}

impl<'a> Iterator for TokenIter<'a> {
	type Item = String;
    
    fn next(&mut self) -> Option<String> {
    	if self.queue.is_empty() { self.parse(); }
    	return if self.queue.is_empty() {
    		None
    	} else { Some(self.queue.pop_front().unwrap()) }
    }
}

struct OpPushDouble {
	pub val: f64
}

impl OpPushDouble {
   	fn new(val: f64) -> Box<Op> {
   		Box::new(OpPushDouble {
   			val: val
   		})
   	}
}

impl Op for OpPushDouble {
	fn apply(&self, ctx: &mut Context) {
		ctx.push(Box::new(Value::Double(self.val)));
	}
}

fn opvec_to_fn(opvec: Vec<Box<Op>>) -> Rc<Fn(&mut Context)> {
	Rc::new(move |ctx| {
		for o in opvec.iter() { o.apply(ctx); }
	})
}

struct OpPushLambda {
	pub val: Rc<Fn(&mut Context)>
}

impl OpPushLambda {
   	fn new(val: Rc<Fn(&mut Context)>) -> Box<Op> {
   		Box::new(OpPushLambda {
   			val: val
   		})
   	}
}

impl Op for OpPushLambda {
	fn apply(&self, ctx: &mut Context) {
		ctx.push(Box::new(Value::Lambda(self.val.clone())));
	}
}

pub fn compile(ops: &Ops, toks: &mut Iterator<Item = String>) -> Vec<Box<Op>> {
	let mut list: Vec<Box<Op>> = Vec::new();

	loop {
		let t = toks.next();
		if let None = t { break; }
		let t = t.unwrap();

		match &t[..] {
		    "{" => list.push(OpPushLambda::new(opvec_to_fn(compile(ops, toks)))),
		    "}" => break,
		    s @ _ => match s.parse::<f64>() {
		    	Ok(n) => list.push(OpPushDouble::new(n)),
		    	_ => list.push(ops.get(s).unwrap())
		    }
		}
	}

	list
}

pub fn interp(ctx: &mut Context, ops: &Ops, code: String) {
	let mut toks = TokenIter::new(&code);
	opvec_to_fn(compile(ops, &mut toks))(ctx);
}