extern crate chattium_oxide_client;
extern crate chattium_oxide_lib;
extern crate hyper;

use hyper::client::Client;
use chattium_oxide_lib::{ChatMessage, ChatUser};
use chattium_oxide_lib::json::ToJsonnable;
use chattium_oxide_client::Options;


fn main() {
	let client = Client::new();
	let options = Options::parse();

	match ChatMessage::new(ChatUser::me(options.name), "Noobel sucks".to_string()).to_json_string() {
		Ok(json) =>
			match client.post(options.server).body(&*&json).send() {
				Ok(response) => println!("Server responded with status {}", response.status),
				Err(error) => println!("POSTing the message failed: {}", error),
			},
		Err(error) => println!("Couldn't serialize message: {}", error),
	};
}
