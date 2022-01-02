use super::utils::iapi::IApi;
use crate::commands::root::Root;
use crate::context::Context;
use crate::request::Request;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

pub struct Console {
	root: Root,
	context: Rc<RefCell<Context>>,
}

impl IApi for Console {
	fn start(&self) {
		let mut input = String::new();
		loop {
			if let Err(error) = io::stdin().read_line(&mut input) {
				println!("{}", error);
				continue;
			};
			let mut cmd_context = Context::new();
			cmd_context.set("user".to_owned(), "root".to_owned());
			cmd_context.set_parent(Some(self.context.clone()));

			match self.root.execute(Request {
				arguments: input.split_whitespace(),
				context: cmd_context,
			}) {
				Ok(response) => println!("{}", response),
				Err(error) => println!("ERROR: {}", error),
			}
			input.clear();
		}
	}
}
impl Console {
	pub fn new(root: Root) -> Self {
		let mut context = Context::new();
		context.set("api".to_owned(), "console".to_owned());

		Self {
			root,
			context: Rc::from(RefCell::from(context)),
		}
	}
}
