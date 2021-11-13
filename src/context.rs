use super::apis::utils::iapi::IApi;
use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;

pub struct Context {
	parent: Option<Rc<Context>>,
	data: HashMap<String, String>,
}

impl Context {
	pub fn set_parent(&mut self, parent: Option<Rc<Context>>) {
		self.parent = parent
	}
	pub fn get(&self, key: String) -> Option<String> {
		let value = self.data.get(&key);
		if let Some(value) = value {
			return Some(value.to_owned());
		}
		None
	}
	pub fn set(&mut self, key: String, value: String) -> Option<String> {
		self.data.insert(key, value)
	}
}

pub fn new() -> Context {
	return Context {
		parent: None,
		data: HashMap::new(),
	};
}
