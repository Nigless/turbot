use crate::context::Context;
use crate::response::Response;
use std::str::Split;

pub trait IApi {
	fn start(&self);
}
