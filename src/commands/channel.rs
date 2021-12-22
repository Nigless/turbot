﻿use super::utils::icommand::ICommand;
use super::utils::subrouter::SubRouter;
use crate::request::Request;
use crate::response::Response;

pub struct Channel {
	subrouter: SubRouter<Self>,
}

impl ICommand for Channel {
	fn execute(&self, request: Request) -> Response {
		match self.subrouter.dispatch(self, request) {
			Some(response) => return response,
			None => return Err("Error".to_owned()),
		}
	}

	fn get_key(&self) -> String {
		"channel".to_owned()
	}
}

impl Channel {
	fn list(&self, request: Request) -> Response {
		Ok("".to_owned())
	}

	pub fn new() -> Self {
		let mut subrouter = SubRouter::new();

		subrouter.register("list", Self::list);

		return Self { subrouter };
	}
}
