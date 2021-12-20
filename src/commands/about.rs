use super::utils::icommand::ICommand;
use crate::response::Response;
use crate::request::Request;

pub struct About {
	key: String,
}

impl ICommand for About {
	fn execute(&self, request:Request) -> Response {
		Ok("Soon...".to_owned())
	}

	fn get_key(&self) -> String {
		self.key.to_owned()
	}
}

pub fn new() -> About {
	return About {
		key: String::from("about"),
	};
}
