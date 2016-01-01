use Options;
use std::sync::{Arc, Mutex};
use std::thread::sleep_ms;
use hyper::client::Client;
use hyper::method::Method;
use time::{now_utc, strftime};
use std::io::{stderr, Read, Write};
use chattium_oxide_lib::ChatMessage;
use chattium_oxide_lib::json::{ToJsonnable, FromJsonnable};


pub fn response_request_loop(options: Options, client: Arc<Client>, keep_going: Arc<Mutex<bool>>) {
	let mut newest = now_utc();
	while *keep_going.lock().unwrap() {
		let just_before = now_utc();
		match newest.to_json_string() {
			Ok(json) =>
				match client.request(Method::Trace, &*&options.server).body(&*&json).send() {
					Ok(mut res) => {
						let mut resbody = String::new();
						match res.read_to_string(&mut resbody) {
							Ok(_) =>
								match Vec::<ChatMessage>::from_json_string(&resbody) {
									Ok(messages) =>
										for message in messages {
											println!("{} | {}: {}", strftime("%T", &message.time_posted).unwrap(), message.sender.name, message.value);
										},
									Err(error) =>
										{let _ = stderr().write_fmt(format_args!("Server at {} replied with unprocessable entity (invalid JSON): {}\n", options.server, error));},
								},
							Err(error) => {let _ = stderr().write_fmt(format_args!("Failed reading request from server at {}: {}\n", res.url, error));},
						}
					},
					Err(error) => {let _ = stderr().write_fmt(format_args!("GETing the message failed: {}\n", error));},
				},
			Err(error) => {let _ = stderr().write_fmt(format_args!("Couldn't serialize message: {}\n", error));},
		}

		newest = just_before;
		sleep_ms(500);
	}
}
