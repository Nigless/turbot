use super::utils::icommand::ICommand;
use super::utils::router::Router;
use crate::request::Request;
use crate::response::Response;

pub struct Channel {
	router: Router<fn(&Self, Request) -> Response>,
}

impl ICommand for Channel {
	fn execute(&self, mut request: Request) -> Response {
		if let Some(key) = request.arguments.next() {
			match self.router.dispatch(key) {
				Some(command) => return command(self, request),
				None => return Err("ERROR".to_owned()),
			}
		}
		Err("".to_owned())
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
		let mut router = Router::<fn(&Self, Request) -> Response>::new();

		router.register("list".to_owned(), Self::list);

		return Self { router };
	}
}
