use super::super::utils::command::Command;
use super::super::utils::router;
use super::super::utils::router::Router;
use super::list;
use crate::response::Response;
use std::str::Split;

pub struct Channel {
	key: String,
	router: Router,
}

impl Command for Channel {
	fn execute(&self, mut arguments: Split<&str>) -> Response {
		match self.router.dispatch(arguments) {
			Some(response) => return response,
			None => return Err("Error".to_string()),
		}
	}

	fn get_key(&self) -> String {
		self.key.to_string()
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
