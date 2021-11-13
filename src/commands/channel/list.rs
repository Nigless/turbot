use super::super::utils::icommand::ICommand;
use crate::context::Context;
use crate::response::Response;
use std::str::Split;

pub struct List {
	key: String,
}

impl ICommand for List {
	fn execute(&self, arguments: Split<&str>, context: Context) -> Response {
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
