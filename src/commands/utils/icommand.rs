use crate::response::Response;
use crate::request::Request;

pub trait ICommand {
	fn execute(&self, request: Request) -> Response;
	fn get_key(&self) -> String;
}
