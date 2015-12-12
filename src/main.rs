extern crate chattium_oxide_lib;
extern crate hyper;

use hyper::client::Client;
use chattium_oxide_lib::{ChatMessage, ChatUser};
use chattium_oxide_lib::json::ToJsonnable;


fn main() {
	let client = Client::new();

	match ChatMessage::new(ChatUser::me("nabijaczleweli".to_string()), "Nooble sucks".to_string()).to_json_string() {
		Ok(json) =>
			match client.post("http://127.0.0.1:50030").body(&*&json).send() {
				Ok(response) => println!("Server responded with status {}", response.status),
				Err(error) => println!("POSTing the message failed: {}", error),
			},
		Err(error) => println!("Couldn't serialize message: {}", error),
	};
}
