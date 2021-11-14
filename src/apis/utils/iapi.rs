use crate::context::Context;
use crate::response::Response;
use async_trait::async_trait;
use std::str::Split;

#[async_trait]
pub trait IApi {
	async fn start(&mut self) -> Response;
	fn stop(&mut self);
	fn execute(&self, arguments: Split<&str>, context: Context) -> Response;
	fn get_name(&self) -> String;
}
