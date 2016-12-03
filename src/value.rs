use std::string::{ToString};
use std::rc::{Rc};

use context::Context;

pub enum Value {
	Null,
	Double(f64),
	List(Vec<Box<Value>>),
	Lambda(Rc<Fn(&mut Context)>)
}

impl Value {

}

impl Clone for Value {
	fn clone(&self) -> Self {
		match *self {
			Value::Null => Value::Null,
			Value::Double(v) => Value::Double(v),
			Value::List(ref v) => Value::List(v.clone()),
			Value::Lambda(ref v) => Value::Lambda(v.clone())
		}
	}
}

impl ToString for Value {
	fn to_string(&self) -> String {
		match *self {
			Value::Null => String::from("null"),
			Value::Double(v) => v.to_string(),
			Value::List(ref list) => {
				let mut o = list.iter().map(|v| v.to_string()).fold(String::from("["), |acc, x| acc + &x + " ");
				o.pop();
				o += "]";
				o
			},
			Value::Lambda(_) => String::from("(lambda)")
		}
	}
}