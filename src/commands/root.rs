use super::about::About;
use super::channel::channel::Channel;
use super::hello::Hello;
use super::utils::router::Router;
use crate::request::Request;
use crate::response::Response;

pub struct Root {
	router: Router,
}

impl Root {
	pub fn execute(&self, request: Request) -> Response {
		match self.router.dispatch(request) {
			Some(response) => return response,
			None => return Err("ERROR".to_owned()),
		}
	}

	pub fn new() -> Self {
		let mut router = Router::new();
		router.register(Box::new(Hello::new()));
		router.register(Box::new(About::new()));
		router.register(Box::new(Channel::new()));

		return Self { router };
	}
}
