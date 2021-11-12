use crate::response::Response;
use std::str::Split;

pub trait IApi {
	fn start(&mut self) -> Response;
	fn stop(&mut self);
	fn execute(&self, arguments: Split<&str>) -> Response;
	fn get_name(&self) -> String;
}
