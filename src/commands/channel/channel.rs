use super::super::utils::icommand::ICommand;
use super::super::utils::router;
use super::super::utils::router::Router;
use super::list;
use crate::response::Response;
use crate::request::Request;

pub struct Channel {
	key: String,
	router: Router,
}

impl ICommand for Channel {
	fn execute(&self, request: Request) -> Response {
		match self.router.dispatch(request) {
			Some(response) => return response,
			None => return Err("Error".to_owned()),
		}
	}

	fn get_key(&self) -> String {
		self.key.to_owned()
	}
}

pub fn new() -> Channel {
	let mut router = router::new();

	router.register(Box::new(list::new()));

	return Channel {
		key: String::from("channel"),
		router,
	};
}
