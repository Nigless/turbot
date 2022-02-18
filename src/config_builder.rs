use merge::Merge;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use toml;
use toml::macros::Deserialize;

pub struct ConfigBuilder<'a> {
	path: PathBuf,
	default_data: &'a str,
}

impl<'a> ConfigBuilder<'a> {
	pub fn new(file_path: &'a str, default_data: &'a str) -> Self {
		let configs_path = "./debug";
		let mut path = PathBuf::from(configs_path);
		path.push(file_path);
		Self {
			path: path,
			default_data: default_data,
		}
	}

	pub fn generate(&self, file_path: Option<&str>) -> Result<(), Box<dyn Error>> {
		let mut buf = PathBuf::new();
		let file_path = file_path.map(|s| {
			buf.push(s);
			&buf
		});
		let mut file = OpenOptions::new()
			.write(true)
			.create(true)
			.open(file_path.unwrap_or(&self.path))?;
		write!(file, "{}", self.default_data);
		Ok(())
	}

	pub fn open<Pattern>(
		&self,
		buff: &'a mut String,
		file_path: Option<&str>,
	) -> Result<Pattern, Box<dyn Error>>
	where
		Pattern: Deserialize<'a> + Merge,
	{
		let mut buf = PathBuf::new();
		let file_path = file_path.map(|s| {
			buf.push(s);
			&buf
		});

		let mut file = File::open(file_path.unwrap_or(&self.path))?;
		file.read_to_string(buff)?;
		let mut config = toml::from_str::<'a, Pattern>(buff)?;
		config.merge(toml::from_str::<'a, Pattern>(self.default_data).unwrap());
		Ok(config)
	}
}
