use super::super::utils::icommand::ICommand;
use crate::request::Request;
use crate::response::Response;

pub struct List {
	key: String,
}

impl ICommand for List {
	fn execute(&self, request: Request) -> Response {
		Ok("Hi!".to_owned())
	}

	fn get_key(&self) -> String {
		self.key.to_owned()
	}
}

impl List {
	pub fn new() -> Self {
		Self {
			key: String::from("list"),
		}
	}
}
