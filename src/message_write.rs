use Options;
use std::io::{stderr, Write};
use std::sync::Arc;
use hyper::client::Client;
use bear_lib_terminal::terminal;
use bear_lib_terminal::geometry::Point;
use chattium_oxide_lib::{ChatMessage, ChatUser};
use chattium_oxide_lib::json::ToJsonnable;


pub struct MessageWriter {
	username: String,
	server: String,
	client: Arc<Client>,
}


impl MessageWriter {
	pub fn new(options: Options, client: Arc<Client>) -> MessageWriter {
		MessageWriter{
			username: options.name,
			server: options.server,
			client: client,
		}
	}

	pub fn call(self) {
		while let Some(rmessage) = terminal::read_str(Point::new(0, terminal::state::size().height - 1), terminal::state::size().width) {
			if !rmessage.is_empty() {
				match ChatMessage::new(ChatUser::me(self.username.clone()), rmessage).to_json_string() {
					Ok(json) =>
						match self.client.post(&*&self.server).body(&*&json).send() {
							Ok(response) => println!("Server responded with status {}", response.status),
							Err(error) => {let _ = stderr().write_fmt(format_args!("POSTing the message failed: {}\n", error));},
						},
					Err(error) => {let _ = stderr().write_fmt(format_args!("Couldn't serialize message: {}\n", error));},
				}
			}
		}
	}
}
