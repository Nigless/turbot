use std::iter::Iterator;
use std::str::Chars;

pub struct Options<'a> {
	string: Chars<'a>,
	next: fn(&mut Self, ch: char) -> Option<String>,
}

impl<'a> Options<'a> {
	pub fn from(string: &'a str) -> Self {
		Self {
			string: string.chars(),
			next: Self::parse,
		}
	}

	fn parse(&mut self, ch: char) -> Option<String> {
		return match ch {
			'-' => self.parse_option(),
			_ => None,
		};
	}

	fn parse_option(&mut self) -> Option<String> {
		if let Some(ch) = self.string.next() {
			return match ch {
				'-' => self.parse_name(),
				_ => {
					self.next = Self::parse_alias;
					self.parse_alias(ch)
				}
			};
		}
		None
	}

	fn parse_name(&mut self) -> Option<String> {
		let mut name = String::new();
		while let Some(ch) = self.string.next() {
			name.push(ch)
		}
		if !name.is_empty() {
			return Some(name);
		}
		None
	}

	fn parse_alias(&mut self, ch: char) -> Option<String> {
		Some(ch.to_string())
	}
}

impl Iterator for Options<'_> {
	type Item = String;
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(ch) = self.string.next() {
			return (self.next)(self, ch);
		}
		None
	}
}
