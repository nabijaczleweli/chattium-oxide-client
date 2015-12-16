use std::fmt::Arguments as FormatArguments;
use std::io::{self, Write};


pub fn read_prompted(prompt: FormatArguments) -> io::Result<Option<String>> {
	try!(io::stdout().write_fmt(prompt));
	try!(io::stdout().flush());

	read_unprompted()
}

pub fn read_unprompted() -> io::Result<Option<String>> {
	let mut obuf = String::new();
	try!(io::stdin().read_line(&mut obuf));  // Don't match on this directly, because it returns with "\r\n"
	let obuf = obuf.trim();
	Ok(match obuf.len() {
		0 => None,
		_ => Some(obuf.trim().to_string()),
	})
}
