use super::utils::iapi::IApi;
use crate::commands::root;
use crate::commands::root::Root;
use crate::commands::utils::icommand::ICommand;
use crate::context;
use crate::context::Context;
use crate::response::Response;
use async_trait::async_trait;
use serenity;
use serenity::client::{Client, EventHandler};
use std::io;
use std::rc::Rc;
use std::str::Split;

pub struct Console {
	is_active: bool,
	name: String,
	root: Rc<Root>,
	context: Rc<Context>,
}

#[async_trait]
impl IApi for Console {
	async fn start(&mut self) -> Response {
		let mut client = Client::builder("token");
		Ok("sdf".to_owned())
	}

	fn stop(&mut self) {
		self.is_active = false;
	}
	fn execute(&self, arguments: Split<&str>, context: Context) -> Response {
		self.root.execute(arguments, context)
	}
	fn get_name(&self) -> String {
		self.name.to_owned()
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
