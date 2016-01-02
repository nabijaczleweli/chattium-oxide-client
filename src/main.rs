extern crate chattium_oxide_lib;
extern crate bear_lib_terminal;
extern crate yaml_file_handler;
extern crate hyper;
extern crate clap;
extern crate time;

mod io;
mod options;
mod response_request;

use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{stderr, Write};
use hyper::client::Client;
use bear_lib_terminal::terminal;
use bear_lib_terminal::geometry::Point;
use bear_lib_terminal::terminal::config;
use chattium_oxide_lib::{ChatMessage, ChatUser};
use chattium_oxide_lib::json::ToJsonnable;
use response_request::ResponseRequester;

pub use options::Options;


fn main() {
	terminal::open("chattium-oxide client", 80, 30);
	terminal::set(config::Window::empty().resizeable(true));
	terminal::refresh();

	let client = Arc::new(Client::new());
	let options = Options::parse();
	let keep_getting_responses = Arc::new(Mutex::new(true));


	let getting_responses_options = options.clone();
	let getting_responses_client = client.clone();
	let getting_responses_going = keep_getting_responses.clone();
	let getting_responses = thread::spawn(move || ResponseRequester::new(getting_responses_options, getting_responses_client, getting_responses_going).call());


	while let Some(rmessage) = terminal::read_str(Point::new(0, 29), 500) {  // Let's assume 500 is the widest someone can exapnd teh terminal.
		if !rmessage.is_empty() {
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


	println!("Terminating...");
	*keep_getting_responses.lock().unwrap() = false;
	if let Err(error) = getting_responses.join() {
		let _ = stderr().write_fmt(format_args!("Response getter thread failed: {:?}\n", error));
	}

	terminal::close();
}
