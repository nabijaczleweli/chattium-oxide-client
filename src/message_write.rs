use Options;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use hyper::client::Client;
use bear_lib_terminal::terminal;
use bear_lib_terminal::geometry::Rect;
use chattium_oxide_lib::{ChatMessage, ChatUser};
use chattium_oxide_lib::json::ToJsonnable;


pub struct MessageWriter {
	username: String,
	server  : String,
	client  : Arc<Client>,
	receiver: Receiver<char>,
}


impl MessageWriter {
	pub fn new(options: Options, client: Arc<Client>, rx: Receiver<char>) -> MessageWriter {
		MessageWriter{
			username: options.name,
			server  : options.server,
			client  : client,
			receiver: rx,
		}
	}

	pub fn call(self) {
		let mut message = "".to_string();

		for event in self.receiver.iter() {
			match event {
				'\u{0}' => (),
				'\u{1}' => break,
				'\n'    => {
					if !message.is_empty() {
						match ChatMessage::new(ChatUser::me(self.username.clone()), message).to_json_string() {
							Ok(json) =>
								match self.client.post(&*&self.server).body(&*&json).send() {
									Ok(response) => println!("Server responded with status {}", response.status),
									Err(error)   => printerr!("POSTing the message failed: {}\n", error),
								},
							Err(error) => printerr!("Couldn't serialize message: {}\n", error),
						}

						message = "".to_string();
						let size = terminal::state::size();
						terminal::clear(Some(Rect::from_point_values(0, size.height - 1, size.width, size.height - 1)));
					}
				},
				'\r' => {
					message.pop();
					Self::print(&message, ' ');
				}
				ch => {
					Self::print(&message, ch);
					message.push(ch);
				},
			}
		}
	}


	fn print(s: &String, ch: char) {
		terminal::put_xy(s.chars().count() as i32, terminal::state::size().height - 1, ch);
		terminal::refresh();
	}
}
