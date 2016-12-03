use value::{Value};

use std::boxed::{Box};
use std::vec::{Drain, IntoIter};
use std::slice::{Iter, IterMut};
use std::iter::{Iterator, Rev, Take};

type FrameIter<'a> = Take<Rev<Iter<'a, Box<Value>>>>;
type FrameMutIter<'a> = Take<Rev<IterMut<'a, Box<Value>>>>;
type FrameDrain<'a> = Rev<Drain<'a, Box<Value>>>;

pub struct StackIter<'a> {
	frames: Iter<'a, Frame>,
	cur: FrameIter<'a>,
}

impl<'a> Iterator for StackIter<'a> {
	type Item = &'a Box<Value>;

	fn next(&mut self) -> Option<&'a Box<Value>> {
		loop {
			match self.cur.next() {
				val @ Some(_) => return val,
				_ => self.cur = match self.frames.next_back() {
					Some(fra) => fra.iter(),
					_ => return None
				}
			}
		}
	}
}

pub struct StackMutIter<'a> {
	frames: IterMut<'a, Frame>,
	cur: FrameMutIter<'a>,
}

impl<'a> Iterator for StackMutIter<'a> {
	type Item = &'a mut Box<Value>;

	fn next(&mut self) -> Option<&'a mut Box<Value>> {
		loop {
			match self.cur.next() {
				val @ Some(_) => return val,
				_ => self.cur = match self.frames.next_back() {
					Some(fra) => fra.iter_mut(),
					_ => return None
				}
			}
		}
	}
}

pub struct StackDrain<'a> {
	drains: IntoIter<FrameDrain<'a>>,
	cur: FrameDrain<'a>
}

impl<'a> Iterator for StackDrain<'a> {

	type Item = Box<Value>;

	fn next(&mut self) -> Option<Box<Value>> {
		loop {
			match self.cur.next() {
				val @ Some(_) => return val,
				_ => self.cur = match self.drains.next() {
					Some(dra) => dra,
					_ => return None
				}
			}
		}
	}
}

pub struct Context {
	frames: Vec<Frame>
}

impl Context {
	pub fn new() -> Context {
		let mut out = Context {
			frames: Vec::new()
		};
		out.enter_frame();
		out
	}

	pub fn top_frame(&mut self) ->  &mut Frame {
		self.frames.last_mut().unwrap()
	}

	pub fn enter_frame(&mut self) {
		self.frames.push(Frame {
			start: 0,
			stack: Vec::new()
		});
	}

	// TODO proper errors
	
	pub fn exit_frame(&mut self) {
		let mut exit = self.frames.pop().unwrap();
		self.top_frame().stack.append(&mut exit.stack);
	}

	pub fn exit_frame_list(&mut self) {
		let mut exit = self.frames.pop().unwrap();
		let mut listval = Vec::new();
		listval.append(&mut exit.stack);
		let list = Value::List(listval);
		self.top_frame().stack.push(Box::new(list));
	}

	pub fn exit_frame_clear(&mut self) {
		self.frames.pop().unwrap();
	}

	pub fn hide_frame(&mut self) {
		self.top_frame().hide();
	}

	pub fn pop(&mut self) -> Option<Box<Value>> {
		for ref mut frame in self.frames.iter_mut().rev() {
			if let v @ Some(_) = frame.pop() { return v; }
		}
		return None;
	}

	pub fn peek(&self) -> Option<&Box<Value>> {
		for frame in self.frames.iter().rev() {
			if let v @ Some(_) = frame.peek() { return v; }
		}
		return None;
	}

	pub fn peek_mut(&mut self) -> Option<&mut Box<Value>> {
		for frame in self.frames.iter_mut().rev() {
			if let v @ Some(_) = frame.peek_mut() { return v; }
		}
		return None;
	}

	pub fn push(&mut self, val: Box<Value>) {
		self.top_frame().push(val);
	}

	pub fn iter(&self) -> StackIter {
		let mut iter = self.frames.iter();
		let cur = iter.next_back().unwrap().iter();
		StackIter {
			frames: iter,
			cur: cur

		}
	}

	pub fn iter_mut<'a>(&'a mut self) -> StackMutIter<'a> {
		let mut iter = self.frames.iter_mut();
		let cur = iter.next_back().unwrap().iter_mut();
		StackMutIter {
			frames: iter,
			cur: cur

		}
	}

	pub fn drain<'a>(&'a mut self, mut count: usize) -> StackDrain<'a> {
		let mut list = Vec::new();
		for frame in self.frames.iter_mut().rev() {
			let dr = frame.drain(count);
			count -= dr.0;
			list.push(dr.1);
			if count <= 0 { break; }
		}

		let mut iter = list.into_iter();
		let cur = iter.next().unwrap();

		return StackDrain {
			drains: iter,
			cur: cur
		}
	}

	pub fn print_stack(&self) -> String {
		let mut out = String::from("<STACK TOP>\n");
		for frame in self.frames.iter().rev() {
			let vis = frame.available();
			let count = frame.stack.len();
			for val in frame.stack[(count - vis)..count].iter().rev() {
				out += &format!("{}\n", val.to_string());
			}
			for val in frame.stack[0..(count - vis)].iter().rev() {
				out += &format!("  * {}\n", val.to_string());
			}
			out += "---------------\n";
		}
		let newlen = out.len() - 17;
		out.truncate(newlen);
		out += "\n<STACK BOTTOM>";
		out
	}
}

pub struct Frame {
	start: usize,
	pub stack: Vec<Box<Value>>
}

impl Frame {
	pub fn hide(&mut self) {
		self.start = self.stack.len(); 
	}

	pub fn peek(&self) -> Option<&Box<Value>> {
		if self.stack.len() <= self.start { return None; }
		return self.stack.last();
	}

	pub fn peek_mut(&mut self) -> Option<&mut Box<Value>> {
		if self.stack.len() <= self.start { return None; }
		return self.stack.last_mut();
	}

	pub fn pop(&mut self) -> Option<Box<Value>> {
		let len = self.stack.len();
		if len <= self.start { return None; }
		if len == self.start - 1 { self.start -= 1 }
		return self.stack.pop();
	}

	pub fn push(&mut self, val: Box<Value>) {
		self.stack.push(val);
	}

	pub fn iter(&self) -> FrameIter {
		self.stack.iter().rev().take(self.stack.len() - self.start)
	}

	pub fn iter_mut(&mut self) -> FrameMutIter {
		let len = self.stack.len();
		self.stack.iter_mut().rev().take(len - self.start)
	}

	fn drain(&mut self, mut count: usize) -> (usize, FrameDrain) {
		let len = self.stack.len();
		let avail = self.available();
		if count > avail { count = avail }
		(count, self.stack.drain((len - count)..len).rev())
	}

	pub fn available(&self) -> usize {
		return self.stack.len() - self.start;
	}
}