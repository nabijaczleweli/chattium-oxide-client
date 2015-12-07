use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
	let mut strm = TcpStream::connect("127.0.0.1:50030").ok().unwrap();
	println!("{:?}", strm);

	println!("{:?}", strm.write(&[0x41]));
	println!("{:?}", strm.flush());

	let mut rr = String::new();
	println!("{:?}", strm.read_to_string(&mut rr).unwrap());
	println!("{}", rr);
}
