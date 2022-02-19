use std::iter::Iterator;
type Dash<'a> = (&'a str, Option<&'a str>);

pub struct Dashes<'a> {
	index: usize,
	string: &'a str,
	parse: fn(&mut Self) -> Option<Dash<'a>>,
	finished: bool,
}

impl<'a> Dashes<'a> {
	pub fn from(string: &'a str) -> Self {
		Self {
			index: 0,
			string: string,
			parse: Self::parse,
			finished: false,
		}
	}

	fn next_str(&mut self) -> Option<&'a str> {
		if self.finished {
			return None;
		}
		self.index = self.index + 1;
		let i = self.index;
		if i >= self.string.len() {
			self.finished = true;
			return None;
		}
		Some(&self.string[i - 1..i])
	}
	
	fn parse(&mut self) -> Option<Dash<'a>> {
		return match self.next_str()? {
			"-" => self.parse_option(),
			_ => None,
		};
	}

	fn parse_option(&mut self) -> Option<Dash<'a>> {
		if let Some(ch) = self.next_str() {
			return match ch {
				"-" => self.parse_name(),
				_ => {
					self.parse = Self::parse_alias;
					self.parse_alias()
				}
			};
		}
		None
	}

	fn parse_name(&mut self) -> Option<Dash<'a>> {
		let start = self.index;
		while let Some(ch) = self.next_str() {
			match ch {
				"=" => {
					if self.index == start + 1 {
						return None;
					}
					return Some((&self.string[start..self.index - 1], self.parse_value()));
				}
				_ => continue,
			}
		}

		if self.index == start {
			return None;
		}

		Some((&self.string[start..self.index], None))
	}

	fn parse_value(&mut self) -> Option<&'a str> {
		Some(&self.string[self.index..])
	}

	fn parse_alias(&mut self) -> Option<Dash<'a>> {
		Some((self.next_str()?, None))
	}
}

impl<'a> Iterator for Dashes<'a> {
	type Item = Dash<'a>;
	fn next(&mut self) -> Option<Self::Item> {
		(self.parse)(self)
	}
}
