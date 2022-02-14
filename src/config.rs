use io::Write;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;
use std::str::Split;
use yaml_rust::ScanError;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

pub struct Config {
	data: Option<Yaml>,
	default: Option<Yaml>,
}

impl Config {
	pub fn new(path: &str, default: &str) -> Result<Self, Box<dyn Error>> {
		let mut file = if Path::new(path).exists() {
			File::open(path)?
		} else {
			let mut file = OpenOptions::new()
				.read(true)
				.write(true)
				.create(true)
				.open(path)?;
			write!(file, "{}", default);
			file
		};
		let mut data = String::new();
		file.read_to_string(&mut data)?;
		Ok(Self {
			data: YamlLoader::load_from_str(&data)?.get(0).map(|i| i.clone()),
			default: YamlLoader::load_from_str(&default)?
				.get(0)
				.map(|i| i.clone()),
		})
	}

	pub fn get(&self, parameter: &str) -> &Yaml {
		if let Some(data) = Self::get_data(self.data.as_ref(), parameter.split(".")) {
			return data;
		}
		if let Some(default) = Self::get_data(self.default.as_ref(), parameter.split(".")) {
			return default;
		}
		panic!("Used a parameter that doesn't exist `{}`", parameter)
	}

	fn get_data<'a>(data: Option<&'a Yaml>, keys: Split<&str>) -> Option<&'a Yaml> {
		let mut data = data?;
		for key in keys {
			data = &data[key];
		}
		return Some(data);
	}
}
