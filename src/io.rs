pub fn maybe_trimmed(buf: String) -> Option<String> {
	trimmed_or_none(buf.trim())
}

pub fn maybe_trimmed_right(buf: &String) -> Option<String> {
	trimmed_or_none(buf.trim_right())
}


macro_rules! printerr {
	($fmt:expr, $($args:tt)*) => {{
		use std::io::{stderr, Write};

		let _ = stderr().write_fmt(format_args!($fmt, $($args)*));
	}}
}


fn trimmed_or_none(buf: &str) -> Option<String> {
	match buf.len() {
		0 => None,
		_ => Some(buf.to_string())
	}
}
