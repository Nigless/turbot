use crate::context::Context;
use crate::response::Response;
use std::str::Split;

pub trait IApi {
	fn start(&mut self);
	fn stop(&mut self);
	fn execute(&self, arguments: Split<&str>, context: Context) -> Response;
	fn get_name(&self) -> String;
}
