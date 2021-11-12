use super::utils::command::Command;
use crate::response::Response;
use std::str::Split;

pub struct Hello {
	key: String,
}

impl Command for Hello {
	fn execute(&self, arguments: Split<&str>) -> Response {
		Ok("Hi!".to_string())
	}

	fn get_key(&self) -> String {
		self.key.to_string()
	}
}

pub fn new() -> Hello {
	Hello {
		key: String::from("hello"),
	}
}
