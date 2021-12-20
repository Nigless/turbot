use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Context {
	parent: Link,
	data: HashMap<String, String>,
}

type Link = Option<Rc<RefCell<Context>>>;

impl Context {
	pub fn set_parent(&mut self, parent: Link) {
		self.parent = parent
	}

	pub fn get(&self, key: String) -> Option<String> {
		if let Some(value) = self.data.get(&key) {
			return Some(value.to_owned());
		};

		if let Some(parent) = &self.parent {
			return parent.borrow().get(key);
		}

		None
	}

	pub fn set(&mut self, key: String, value: String) -> Option<String> {
		if self.data.contains_key(&key) {
			return self.data.insert(key, value);
		};

		if let Some(parent) = &self.parent {
			return parent.borrow_mut().set(key, value);
		}

		None
	}

	pub fn new() -> Self {
		return Self {
			parent: None,
			data: HashMap::new(),
		};
	}
}
