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
	Ok(maybe_trimmed(obuf))
}

pub fn maybe_trimmed(buf: String) -> Option<String> {
	let buf = buf.trim();
	match buf.len() {
		0 => None,
		_ => Some(buf.to_string())
	}
}
