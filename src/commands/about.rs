use super::utils::icommand::ICommand;
use crate::request::Request;
use crate::response::Response;

pub struct About {
	key: String,
}

impl ICommand for About {
	fn execute(&self, request: Request) -> Response {
		Ok("Soon...".to_owned())
	}

	fn get_key(&self) -> String {
		self.key.to_owned()
	}
}

impl About {
	pub fn new() -> Self {
		return Self {
			key: String::from("about"),
		};
	}
}
