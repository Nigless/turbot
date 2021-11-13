use super::utils::iapi::IApi;
use crate::commands::root;
use crate::commands::root::Root;
use crate::commands::utils::icommand::ICommand;
use crate::context;
use crate::context::Context;
use crate::response::Response;
use std::io;
use std::rc::Rc;
use std::str::Split;

pub struct Console {
	is_active: bool,
	name: String,
	root: Rc<Root>,
	context: Rc<Context>,
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
			let mut cmdcontext = context::new();
			cmdcontext.set("user".to_owned(), "root".to_owned());
			cmdcontext.set_parent(Some(self.context.clone()));

			match self.execute(input.split(" "), cmdcontext) {
				Ok(response) => println!("{}", response),
				Err(error) => println!("ERROR: {}", error),
			}
			input.clear();
		}
	}

	fn stop(&mut self) {
		self.is_active = false;
	}
	fn execute(&self, arguments: Split<&str>, context: Context) -> Response {
		self.root.execute(arguments, context)
	}
	fn get_name(&self) -> String {
		self.name.to_string()
	}
}

pub fn new(root: Rc<Root>, is_active: bool) -> Console {
	let mut context = context::new();
	context.set("api".to_owned(), "console".to_owned());

	let mut console = Console {
		is_active: false,
		name: String::from("console"),
		root,
		context: Rc::from(context),
	};
	if is_active {
		console.start();
	}
	console
}
