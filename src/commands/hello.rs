use super::utils::icommand::ICommand;
use crate::request::Request;
use crate::response::Response;

pub struct Hello {
	key: String,
}

impl ICommand for Hello {
	fn execute(&self, request: Request) -> Response {
		Ok("Hi!".to_owned())
	}

	fn get_key(&self) -> String {
		self.key.to_owned()
	}
}

impl Hello {
	pub fn new() -> Self {
		Self {
			key: String::from("hello"),
		}
	}
}
