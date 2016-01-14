pub fn maybe_trimmed(buf: String) -> Option<String> {
	let buf = buf.trim();
	match buf.len() {
		0 => None,
		_ => Some(buf.to_string())
	}
}
