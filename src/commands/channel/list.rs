use super::super::utils::icommand::ICommand;
use crate::response::Response;
use crate::request::Request;

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

pub fn new() -> List {
	List {
		key: String::from("list"),
	}
}
