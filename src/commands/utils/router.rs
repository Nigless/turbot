use super::command::Command;
use super::command::Response;
use std::collections::HashMap;
use std::str::Split;

pub struct Router {
	map: HashMap<String, Box<dyn Command>>,
}

impl Router {
	pub fn register(&mut self, command: Box<dyn Command>) {
		self.map.insert(command.get_key(), command);
	}

	pub fn dispatch(&self, mut arguments: Split<&str>) -> Response {
		if let Some(key) = arguments.next() {
			if let Some(command) = self.map.get(key.trim()) {
				return command.execute(arguments);
			}
			return Err("unknown command".to_string());
		}
		return Err("Oops".to_string());
	}
}

pub fn new() -> Router {
	Router {
		map: HashMap::new(),
	}
}
