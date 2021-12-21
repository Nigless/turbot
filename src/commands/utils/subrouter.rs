use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;

type SubCommand<Owner> = fn(owner: &Owner, request: Request) -> Response;

pub struct SubRouter<Owner> {
	map: HashMap<String, SubCommand<Owner>>,
}

impl<Owner> SubRouter<Owner> {
	pub fn register(&mut self, key: &str, command: SubCommand<Owner>) {
		self.map.insert(key.to_owned(), command);
	}

	pub fn dispatch(&self, owner: &Owner, request: Request) -> Option<Response> {
		let (mut arguments, context) = request;
		if let Some(key) = arguments.next() {
			if let Some(command) = self.map.get(key.trim()) {
				return Some(command(owner, (arguments, context)));
			}
			return Some(Err("unknown subcommand".to_owned()));
		}
		return None;
	}

	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
		}
	}
}
