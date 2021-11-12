use super::utils::api::IApi;
use crate::commands::root;
use crate::commands::root::Root;
use crate::commands::utils::command::Command;
use crate::response::Response;
use std::io;
use std::rc::Rc;
use std::str::Split;

pub struct Console {
	is_active: bool,
	name: String,
	root: Rc<Root>,
}

impl IApi for Console {
	fn start(&mut self) -> Response {
		self.is_active = true;
		let mut input = String::new();
		loop {
			if !self.is_active {
				return Ok(String::from(""));
			}
			if let Err(error) = io::stdin().read_line(&mut input) {
				println!("{}", error);
				continue;
			};

			match self.execute(input.split(" ")) {
				Ok(response) => println!("{}", response),
				Err(error) => println!("ERROR: {}", error),
			}
			input.clear();
		}
	}

	fn stop(&mut self) {
		self.is_active = false;
	}
	fn execute(&self, arguments: Split<&str>) -> Response {
		self.root.execute(arguments)
	}
	fn get_name(&self) -> String {
		self.name.to_string()
	}
}

pub fn new(root: Rc<Root>, is_active: bool) -> Console {
	let mut console = Console {
		is_active: false,
		name: String::from("console"),
		root,
	};
	if is_active {
		console.start();
	}
	console
}
