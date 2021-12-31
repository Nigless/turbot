use super::icommand::ICommand;
use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;

pub struct Router<Type> {
	map: HashMap<String, Type>,
}

impl<Type> Router<Type> {
	pub fn register(&mut self, key: String, command: Type) {
		self.map.insert(key, command);
	}

	pub fn dispatch(&self, key: &str) -> Option<&Type> {
		self.map.get(key.trim())
	}

	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
		}
	}
}
