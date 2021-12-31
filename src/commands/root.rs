use super::channel::Channel;
use super::hello::Hello;
use super::utils::icommand::ICommand;
use super::utils::router::Router;
use crate::request::Request;
use crate::response::Response;

pub struct Root {
	router: Router<Box<dyn ICommand>>,
}

impl Root {
	pub fn execute(&self, mut request: Request) -> Response {
		if let Some(key) = request.0.next() {
			match self.router.dispatch(key) {
				Some(command) => return command.execute(request),
				None => return Err("ERROR".to_owned()),
			}
		}
		Err("".to_owned())
	}

	pub fn new() -> Self {
		let mut router = Router::<Box<dyn ICommand>>::new();
		let hello = Hello::new();
		router.register(hello.get_key(), Box::new(hello));
		let channel = Channel::new();
		router.register(channel.get_key(), Box::new(channel));

		return Self { router };
	}
}
