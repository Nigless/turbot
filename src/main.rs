mod apis;
pub mod commands;
mod context;
pub mod message;
mod request;
mod response;
use apis::console::Console;
use apis::utils::iapi::IApi;
use commands::root::Root;
mod config_builder;
use config_builder::ConfigBuilder;
use std::env;
mod dashes;
use dashes::Dashes;
use merge::Merge;
use serde_derive::Deserialize;

#[derive(Merge, Deserialize)]
struct Config {
	name: Option<String>,
	#[merge(skip)]
	version: String,
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut config_path = None;
	let config = ConfigBuilder::new(
		"conf.toml",
		r#"name = "rust"
version = "0.1.0""#,
	);

	for arg in &args[1..] {
		for option in Dashes::from(arg) {
			match option {
				("generate-config", path) | ("g", path) => config.generate(path).unwrap(),
				("config-path", path) => config_path = Some(path.unwrap()),
				(option, _) => return println!("unknown option: {}", option),
			};
		}
	}

	let config = config
		.open::<Config>(&mut String::new(), config_path)
		.unwrap();

	let root = Root::new();
	let console = Console::new(root);
	console.start();
}
