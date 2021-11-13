use super::utils::icommand::ICommand;
use crate::context::Context;
use crate::response::Response;
use std::str::Split;

pub struct About {
	key: String,
}

impl ICommand for About {
	fn execute(&self, mut arguments: Split<&str>, context: Context) -> Response {
		Ok("Soon...".to_string())
	}

	fn get_key(&self) -> String {
		self.key.to_string()
	}
}

pub fn new() -> About {
	return About {
		key: String::from("about"),
	};
}
