use super::super::utils::icommand::ICommand;
use super::super::utils::router::Router;
use super::list::List;
use crate::request::Request;
use crate::response::Response;

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

impl Channel {
	pub fn new() -> Self {
		let mut router = Router::new();

		router.register(Box::new(List::new()));

		return Self {
			key: String::from("channel"),
			router,
		};
	}
}
