use super::utils::icommand::ICommand;
use crate::response::Response;
use crate::request::Request;

pub struct Hello {
	key: String,
}

impl ICommand for Hello {
	fn execute(&self, request:Request) -> Response {
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
