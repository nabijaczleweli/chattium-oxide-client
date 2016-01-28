use Options;
use io::maybe_trimmed_right;
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
					if let Some(msg) = maybe_trimmed_right(&message) {
						match ChatMessage::new(ChatUser::me(self.username.clone()), msg).to_json_string() {
							Ok(json) =>
								match self.client.post(&*&self.server).body(&*&json).send() {
									Ok(response) => println!("Server responded with status {}", response.status),
									Err(error)   => printerr!("POSTing the message failed: {}\n", error),
								},
							Err(error) => printerr!("Couldn't serialize message: {}\n", error),
						}

						let size = terminal::state::size();
						message = "".to_string();
						terminal::clear(Some(Rect::from_point_values(0, size.height - 1, size.width, size.height - 1)));
						terminal::refresh();
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
