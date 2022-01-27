use super::utils::icommand::ICommand;
use super::utils::options::Options;
use super::utils::router::Router;
use crate::request::Request;
use crate::response::Response;

pub struct Echo {}

impl ICommand for Echo {
	fn execute(&self, mut request: Request) -> Response {
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
