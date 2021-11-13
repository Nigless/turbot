use super::about;
use super::channel::channel;
use super::hello;
use super::utils::icommand::ICommand;
use super::utils::router;
use super::utils::router::Router;
use crate::context::Context;
use crate::response::Response;
use std::str::Split;

pub struct Root {
	key: String,
	router: Router,
}

impl ICommand for Root {
	fn execute(&self, arguments: Split<&str>, context: Context) -> Response {
		match self.router.dispatch(arguments, context) {
			Some(response) => return response,
			None => return Err("ERROR".to_string()),
		}
	}

	fn get_key(&self) -> String {
		self.key.to_string()
	}
}

pub fn new() -> Root {
	let mut router = router::new();

	router.register(Box::new(hello::new()));
	router.register(Box::new(about::new()));
	router.register(Box::new(channel::new()));

	return Root {
		key: String::new(),
		router,
	};
}
