mod apis;
pub mod commands;
mod context;
pub mod message;
mod request;
mod response;
use apis::console::Console;
use apis::utils::iapi::IApi;
use commands::root::Root;

fn main() {
	let root = Root::new();
	let console = Console::new(root);
	console.start()
}
