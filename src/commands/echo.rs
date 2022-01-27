use super::utils::icommand::ICommand;
use crate::request::Request;
use crate::response::Response;

pub struct Echo {}

impl ICommand for Echo {
	fn execute(&self, request: Request) -> Response {
		Ok(request.arguments.collect::<Vec<&str>>().join(" "))
	}

	fn get_key(&self) -> String {
		"echo".to_owned()
	}
}

impl Echo {
	pub fn new() -> Self {
		Self {}
	}
}
