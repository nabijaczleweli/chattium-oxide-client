use Options;
use std::sync::{Arc, RwLock};
use std::thread::sleep_ms;
use hyper::client::Client;
use hyper::method::Method;
use time::{now_utc, strftime};
use std::io::{stderr, Read, Write};
use bear_lib_terminal::terminal;
use bear_lib_terminal::geometry::{Size, Rect, Point};
use chattium_oxide_lib::ChatMessage;
use chattium_oxide_lib::json::{ToJsonnable, FromJsonnable};


pub struct ResponseRequester {
	server: String,
	client: Arc<Client>,
	keep_going: Arc<RwLock<bool>>,
	terminal_size: Arc<RwLock<Size>>,
	messages: Vec<ChatMessage>,
}


impl ResponseRequester {
	pub fn new(options: Options, client: Arc<Client>, keep_going: Arc<RwLock<bool>>, term_size: Arc<RwLock<Size>>) -> ResponseRequester {
		ResponseRequester{
			server: options.server,
			client: client,
			terminal_size: term_size,
			keep_going: keep_going,
			messages: Vec::new(),
		}
	}

	pub fn call(mut self) {
		self.draw_line();

		let mut newest = now_utc();
		while *self.keep_going.read().unwrap() {
			let just_before = now_utc();
			match newest.to_json_string() {
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
										Err(error) =>
											{let _ = stderr().write_fmt(format_args!("Server at {} replied with unprocessable entity (invalid JSON): {}\n", self.server, error));},
									},
								Err(error) => {let _ = stderr().write_fmt(format_args!("Failed reading request from server at {}: {}\n", res.url, error));},
							}
						},
						Err(error) =>
							{let _ = stderr().write_fmt(format_args!("GETing new (before {}) messages failed: {}\n", strftime("%D %T", &newest).unwrap(), error));},
					},
				Err(error) => {let _ = stderr().write_fmt(format_args!("Couldn't serialize message: {}\n", error));},
			}

			newest = just_before;
			sleep_ms(500);
		}
	}


	fn print_messages(&self) {
		let size = *self.terminal_size.read().unwrap();
		terminal::clear(Some(Rect::from_size(Point::new(0, 0), Size::new(size.width, size.height - 2))));
		for (i, message) in self.messages.iter().rev().take((size.height - 2) as usize).enumerate() {
			terminal::print_xy(0, size.height - 3 - i as i32,
			                   &*&format!("{} | {}: {}", strftime("%T", &message.time_posted).unwrap(), message.sender.name, message.value));
		}
		terminal::refresh();
	}

	fn draw_line(&self) {
		let size = *self.terminal_size.read().unwrap();
		for x in 0..size.width {
			terminal::put_xy(x, size.height - 2, '—')
		}
		terminal::refresh();
	}
}
