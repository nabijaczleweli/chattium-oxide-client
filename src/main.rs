extern crate chattium_oxide_lib;
extern crate bear_lib_terminal;
extern crate yaml_file_handler;
extern crate hyper;
extern crate clap;
extern crate time;

mod io;
mod options;
mod message_write;
mod response_request;

use std::io::{stderr, Write};
use std::sync::{Arc, RwLock};
use std::thread;
use hyper::client::Client;
use message_write::MessageWriter;
use response_request::ResponseRequester;
use bear_lib_terminal::terminal::{self, config};

pub use options::Options;


fn main() {
	terminal::open("chattium-oxide client", 80, 30);
	terminal::set(config::Window::empty().resizeable(true));
	terminal::refresh();

	let client           = Arc::new(Client::new());
	let options          = Options::parse();
	let continue_threads = Arc::new(RwLock::new(true));

	terminal::set(config::Window::empty().title(format!("chattium-oxide client — Connected to {} as {}", options.server, options.name)));


	let writing_messages = {
		let writing_messages_options = options.clone();
		let writing_messages_client  = client.clone();

		thread::spawn(move || MessageWriter::new(writing_messages_options, writing_messages_client).call())
	};

	let getting_responses = {
		let getting_responses_options = options.clone();
		let getting_responses_client  = client.clone();
		let getting_responses_going   = continue_threads.clone();

		thread::spawn(move || ResponseRequester::new(getting_responses_options, getting_responses_client, getting_responses_going).call())
	};


	for _ in terminal::events() {}


	*continue_threads.write().unwrap() = false;

	if let Err(error) = getting_responses.join() {
		let _ = stderr().write_fmt(format_args!("Response getter thread failed: {:?}\n", error));
	}
	if let Err(error) = writing_messages.join() {
		let _ = stderr().write_fmt(format_args!("Message writer thread failed: {:?}\n", error));
	}

	terminal::close();
}
