mod apis;
pub mod commands;
mod context;
pub mod message;
mod response;
use apis::console;
use commands::root;
use std::rc::Rc;

fn main() {
	let root = Rc::from(root::new());
	console::new(root);
}
