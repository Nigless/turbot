use super::super::utils::command::Command;
use crate::response::Response;
use std::str::Split;

pub struct List {
	key: String,
}

impl Command for List {
	fn execute(&self, arguments: Split<&str>) -> Response {
		Ok("Hi!".to_string())
	}

	fn get_key(&self) -> String {
		self.key.to_string()
	}
}

pub fn new() -> List {
	List {
		key: String::from("list"),
	}
}
