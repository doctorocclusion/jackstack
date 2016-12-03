use std::string::{ToString};
use std::fmt::{Display};

use context::Context;

pub enum Value {
	Null,
	Double(f64),
	List(Vec<Box<Value>>),
	Lambda(Box<Fn(&mut Context)>)
}

impl Value {

}

impl ToString for Value {
	fn to_string(&self) -> String {
		match *self {
			Value::Null => String::from("null"),
			Value::Double(v) => v.to_string(),
			Value::List(ref list) => {
				let mut o = list.iter().map(|v| v.to_string()).fold(String::from("["), |acc, x| acc + &x + ", ");
				let len = o.len();
				o.truncate(len - 2);
				o += "]";
				o
			},
			Value::Lambda(_) => String::from("(lambda)")
		}
	}
}