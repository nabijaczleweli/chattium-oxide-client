pub fn maybe_trimmed(buf: String) -> Option<String> {
	let buf = buf.trim();
	match buf.len() {
		0 => None,
		_ => Some(buf.to_string())
	}
}


macro_rules! printerr {
	($fmt:expr, $($args:tt)*) => {{
		use std::io::{stderr, Write};

		let _ = stderr().write_fmt(format_args!($fmt, $($args)*));
	}}
}
