use crate::message::Message;
use std::str::Split;

pub type Response = Result<Message, String>;

pub trait Command {
	fn execute(&self, arguments: Split<&str>) -> Response;
	fn get_key(&self) -> String;
}
