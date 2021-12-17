mod apis;
pub mod commands;
mod context;
pub mod message;
mod response;
use apis::console;
use apis::utils::iapi::IApi;
use commands::root;

fn main() {
	let root = root::new();
	let console = console::new(root);
	console.start()
}
