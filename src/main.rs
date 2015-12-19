extern crate chattium_oxide_lib;
extern crate yaml_file_handler;
extern crate hyper;
extern crate clap;
extern crate time;

mod io;
mod options;

use options::Options;
use io::read_unprompted;
use hyper::client::Client;
use hyper::method::Method;
use time::{now_utc, strftime};
use std::io::{stderr, Read, Write};
use chattium_oxide_lib::{ChatMessage, ChatUser};
use chattium_oxide_lib::json::{ToJsonnable, FromJsonnable};


fn main() {
	let client = Client::new();
	let options = Options::parse();

	let mut newest = now_utc();
	loop {
		use std::thread::sleep_ms;

		let just_before = now_utc();
		match newest.to_json_string() {
			Ok(json) => {
				match client.request(Method::Trace, &*&options.server).body(&*&json).send() {
					Ok(mut res) => {
						let mut resbody = String::new();
						match res.read_to_string(&mut resbody) {
							Ok(_) =>
								match Vec::<ChatMessage>::from_json_string(&resbody) {
									Ok(messages) =>
										for message in messages {
											println!("{}: {} @ {}", message.sender.name, message.value, strftime("%T", &message.time_posted).unwrap());
										},
									Err(error) =>
										{let _ = stderr().write_fmt(format_args!("Server at {} replied with unprocessable entity (invalid JSON): {}\n", options.server, error));},
								},
							Err(error) => {let _ = stderr().write_fmt(format_args!("Failed reading request from server at {}: {}\n", res.url, error));},
						}
					},
					Err(error) => {let _ = stderr().write_fmt(format_args!("GETing the message failed: {}\n", error));},
				}
			},
			Err(error) => {let _ = stderr().write_fmt(format_args!("Couldn't serialize message: {}\n", error));},
		}

		newest = just_before;
		sleep_ms(500);
	}
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
