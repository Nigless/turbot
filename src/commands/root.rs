use super::about;
use super::hello;
use super::utils::command::Command;
use super::utils::command::Response;
use super::utils::router;
use super::utils::router::Router;
use std::str::Split;

pub struct Root {
	key: String,
	router: Router,
}

impl Command for Root {
	fn execute(&self, arguments: Split<&str>) -> Response {
		self.router.dispatch(arguments)
	}

	fn get_key(&self) -> String {
		self.key.to_string()
	}
}

pub fn new() -> Root {
	let mut router = router::new();

	router.register(Box::new(hello::new()));
	router.register(Box::new(about::new()));

	return Root {
		key: String::new(),
		router,
	};
}
