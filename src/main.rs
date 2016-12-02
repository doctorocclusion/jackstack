#![allow(dead_code)]

mod context;
mod value;

use context::{Context};
use value::{Value};

fn print(con: &Context) {
	let mut stack : Vec<&Box<Value>> = con.iter().collect();
	stack.reverse();
    println!("{:?}", stack);
}

fn main() {

    let mut con = Context::new();

    con.push(Box::new(Value::Double(10f64)));

    print(&con);

    con.enter_frame();
    for i in 0..5 {
    	con.push(Box::new(Value::Double(i as f64)));
    }

    print(&con);

    con.hide_frame();

    print(&con);

    for i in 5..10 {
    	con.push(Box::new(Value::Double(i as f64)));
    }

    print(&con);

    con.enter_frame();

    print(&con);

    con.push(Box::new(Value::Double(50f64)));

    print(&con);

    con.hide_frame();

    print(&con);

    con.push(Box::new(Value::Double(60f64)));

    print(&con);

    con.drain(3);

    print(&con);

    let popped = con.pop().unwrap();

    print(&con);

    con.push(popped);

    print(&con);

    con.push(Box::new(Value::Double(40f64)));

    print(&con);

    con.exit_frame_list();

    print(&con);

    con.exit_frame();

    print(&con);
}
