use super::super::utils::icommand::ICommand;
use super::super::utils::router;
use super::super::utils::router::Router;
use super::list;
use crate::context::Context;
use crate::response::Response;
use std::str::Split;

pub struct Channel {
	key: String,
	router: Router,
}

impl ICommand for Channel {
	fn execute(&self, mut arguments: Split<&str>, context: Context) -> Response {
		match self.router.dispatch(arguments, context) {
			Some(response) => return response,
			None => return Err("Error".to_owned()),
		}
	}

	fn get_key(&self) -> String {
		self.key.to_owned()
	}
}

pub fn new() -> Channel {
	let mut router = router::new();

	router.register(Box::new(list::new()));

	return Channel {
		key: String::from("channel"),
		router,
	};
}
