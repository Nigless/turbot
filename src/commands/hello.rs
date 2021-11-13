use super::utils::icommand::ICommand;
use crate::context::Context;
use crate::response::Response;
use std::str::Split;

pub struct Hello {
	key: String,
}

impl ICommand for Hello {
	fn execute(&self, arguments: Split<&str>, context: Context) -> Response {
		Ok("Hi!".to_owned())
	}

	fn get_key(&self) -> String {
		self.key.to_owned()
	}
}

pub fn new() -> Hello {
	Hello {
		key: String::from("hello"),
	}
}
