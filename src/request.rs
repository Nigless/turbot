use super::context::Context; 
use std::str::SplitWhitespace;

pub type Request<'a> = (SplitWhitespace<'a>,Context);