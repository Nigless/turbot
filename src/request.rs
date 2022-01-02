use super::context::Context;
use std::str::SplitWhitespace;

pub struct Request<'a> {
	pub arguments: SplitWhitespace<'a>,
	pub context: Context,
}
