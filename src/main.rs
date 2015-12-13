extern crate chattium_oxide_lib;
extern crate yaml_file_handler;
extern crate hyper;
extern crate clap;

mod options;

use options::Options;
use hyper::client::Client;
use chattium_oxide_lib::{ChatMessage, ChatUser};
use chattium_oxide_lib::json::ToJsonnable;


fn main() {
	let client = Client::new();
	let options = Options::parse();

	match ChatMessage::new(ChatUser::me(options.name.clone()), "Noobel sucks".to_string()).to_json_string() {
		Ok(json) =>
			match client.post(&*&options.server).body(&*&json).send() {
				Ok(response) => println!("Server responded with status {}", response.status),
				Err(error) => println!("POSTing the message failed: {}", error),
			},
		Err(error) => println!("Couldn't serialize message: {}", error),
	};
}
