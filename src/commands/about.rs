use super::utils::command::Command;
use crate::response::Response;
use std::str::Split;

pub struct About {
	key: String,
}

impl Command for About {
	fn execute(&self, mut arguments: Split<&str>) -> Response {
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
