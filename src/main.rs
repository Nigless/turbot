mod apis;
pub mod commands;
mod context;
pub mod message;
mod response;
mod request;
use apis::console;
use apis::utils::iapi::IApi;
use commands::root;

fn main() {
	let root = root::Root::new();
	let console = console::new(root);
	console.start()
}
