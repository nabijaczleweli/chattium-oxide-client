extern crate chattium_oxide_lib;
extern crate yaml_file_handler;
extern crate hyper;
extern crate clap;

mod io;
mod options;

use io::read_unprompted;
use options::Options;
use hyper::client::Client;
use std::io::{stderr, Write};
use chattium_oxide_lib::{ChatMessage, ChatUser};
use chattium_oxide_lib::json::ToJsonnable;


fn main() {
	let client = Client::new();
	let options = Options::parse();

	while let Ok(Some(rmessage)) = read_unprompted() {
		match ChatMessage::new(ChatUser::me(options.name.clone()), rmessage).to_json_string() {
			Ok(json) =>
				match client.post(&*&options.server).body(&*&json).send() {
					Ok(response) => println!("Server responded with status {}", response.status),
					Err(error) => {let _ = stderr().write_fmt(format_args!("POSTing the message failed: {}\n", error));},
				},
			Err(error) => {let _ = stderr().write_fmt(format_args!("Couldn't serialize message: {}\n", error));},
		}
	}
}
