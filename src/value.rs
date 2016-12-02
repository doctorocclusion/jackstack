use std::string::{ToString};

#[derive(Debug)]
pub enum Value {
	Null,
	Double(f64),
	List(Vec<Box<Value>>)
}

impl Value {

}

impl ToString for Value {
	fn to_string(&self) -> String {
		match self {
			&Value::Null => String::from("null"),
			&Value::Double(v) => v.to_string(),
			&Value::List(ref v) => String::from("list")
		}
	}
}