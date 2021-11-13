use crate::context::Context;
use crate::response::Response;
use std::str::Split;

pub trait ICommand {
	fn execute(&self, arguments: Split<&str>, context: Context) -> Response;
	fn get_key(&self) -> String;
}
