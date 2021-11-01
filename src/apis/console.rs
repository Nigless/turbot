use crate::commands::root;
use crate::commands::utils::command::Command;
use std::io;
pub fn new() {
	start();
}

fn start() {
	let mut input = String::new();
	let root = root::new();
	loop {
		if let Err(error) = io::stdin().read_line(&mut input) {
			println!("{}", error);
			continue;
		};

		match root.execute(input.split(" ")) {
			Ok(response) => println!("{}", response),
			Err(error) => println!("ERROR: {}", error),
		}
		input.clear();
	}
}
