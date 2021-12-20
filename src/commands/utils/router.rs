use super::icommand::ICommand;
use crate::response::Response;
use std::collections::HashMap;
use crate::request::Request;

pub struct Router {
	map: HashMap<String, Box<dyn ICommand>>,
}

impl Router {
	pub fn register(&mut self, command: Box<dyn ICommand>) {
		self.map.insert(command.get_key(), command);
	}

	pub fn dispatch(&self, request: Request) -> Option<Response> {
		let (mut arguments,context) = request;
		if let Some(key) = arguments.next() {
			if let Some(command) = self.map.get(key.trim()) {
				return Some(command.execute((arguments,context)));
			}
			return Some(Err("unknown command".to_owned()));
		}
		return None;
	}
}

pub fn new() -> Router {
	Router {
		map: HashMap::new(),
	}
}
