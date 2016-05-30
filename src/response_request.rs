use chattium_oxide_lib::json::{ToJsonnable, FromJsonnable, JsonError};
use chattium_oxide_lib::{ChatMessage, ChatUser};
use bear_lib_terminal::geometry::{Size, Rect, Point};
use bear_lib_terminal::{terminal, Color};
use std::io::Read;
use hyper::method::Method;
use hyper::client::Client;
use time::strftime;
use std::thread::sleep;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use Options;


pub struct ResponseRequester {
	server    : String,
	me        : ChatUser,
	time      : Duration,
	client    : Arc<Client>,
	keep_going: Arc<RwLock<bool>>,
	messages  : Vec<ChatMessage>,
}


impl ResponseRequester {
	pub fn new(options: Options, client: Arc<Client>, keep_going: Arc<RwLock<bool>>) -> ResponseRequester {
		ResponseRequester{
			server    : options.server,
			me        : ChatUser::me(options.name),
			time      : options.time,
			client    : client,
			keep_going: keep_going,
			messages  : Vec::new(),
		}
	}

	pub fn call(mut self) {
		self.draw_line();

		while *self.keep_going.read().unwrap() {
			self.draw_line();

			match self.request_message() {
				Ok(json) =>
					match self.client.request(Method::Trace, &*&self.server).body(&*&json).send() {
						Ok(mut res) => {
							let mut resbody = String::new();
							match res.read_to_string(&mut resbody) {
								Ok(_) =>
									match Vec::<ChatMessage>::from_json_string(&resbody) {
										Ok(ref mut messages) => {
											self.messages.append(messages);
											self.print_messages();
										},
										Err(error) => printerr!("Server at {} replied with invalid JSON: {}\n", self.server, error),
									},
								Err(error) => printerr!("Failed reading request from server at {}: {}\n", res.url, error),
							}
						},
						Err(error) => printerr!("GETing new (before #{}) messages failed: {}\n", json, error),
					},
				Err(error) => printerr!("Couldn't serialize message: {}\n", error),
			}

			sleep(self.time);
		}
	}


	fn print_messages(&self) {
		let size = terminal::state::size();

		if size.height != 0 {  // Terminal dead otherwise
			terminal::clear(Some(Rect::from_size(Point::new(0, 0), Size::new(size.width, size.height - 2))));
			for (i, message) in self.messages.iter().rev().take((size.height - 2) as usize).enumerate() {
				let is_own = message.sender == self.me;

				if is_own {
					terminal::set_foreground(Color::from_rgb(221, 221, 221));
				}

				terminal::print_xy(0, size.height - 3 - i as i32,
				                   &*&format!("{} | {}: {}", strftime("%T", &message.time_posted).unwrap(), message.sender.name, message.value));

				if is_own {
					terminal::set_foreground(Color::from_rgb(255, 255, 255));
				}
			}
			terminal::refresh();
		}
	}

	fn draw_line(&self) {
		let size = terminal::state::size();
		for x in 0..size.width {
			terminal::put_xy(x, size.height - 2, '—')
		}
		terminal::refresh();
	}

	fn request_message(&self) -> Result<String, JsonError> {
		self.messages.iter().rev().next().map(|ref m| m.id).unwrap_or(0).to_json_string()
	}
}
