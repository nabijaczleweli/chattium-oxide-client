extern crate hyper;

use hyper::client::Client;


fn main() {
	let client = Client::new();

	match client.post("http://127.0.0.1:50030").body("BODY").send() {
		Ok(response) => println!("Server responded with status {}", response.status),
		Err(error) => println!("Error {}", error),
	}
}
