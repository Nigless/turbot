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
	let config = ConfigBuilder::new(
		"conf.toml",
		r#"name = "rust"
		version = "0.1.0""#,
	);

	let args = env::args().skip(1).collect::<Vec<String>>();
	let mut args = args.iter().map(|x| x.as_str());

	let mut config_path = None;
	while let Some(arg) = args.next() {
		for option in Dashes::from(arg) {
			match option {
				("generate-config", path) => config.generate(path).unwrap(),
				("g", _) => config.generate(args.next()).unwrap(),
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
