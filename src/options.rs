use yaml_file_handler::yaml_handler::FileHandler as YamlFileHandler;
use bear_lib_terminal::geometry::{Point, Rect};
use bear_lib_terminal::terminal;
use clap::App as Clapp;
use std::cmp::max;
use std::str::FromStr;
use std::net::ToSocketAddrs;
use std::env::home_dir;
use std::path::PathBuf;
use std::time::Duration;
use io;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
	pub name   : String,
	pub server : String,
	pub splash : i32,
	pub time   : Duration,
}


impl Options {
	/// Parses commandline arguments into an [`Options`](#) instance
	///
	/// Optionally reads from a config file in [YAML](http://yaml.org) format, however commandline arguments take preference thereover.
	/// The config file format is trivial: all root keys and values are of the same name and format as long commandline arguments,
	/// see `"example/config.yml"`.
	pub fn parse() -> Options {
		const USAGE: &'static str = "-c --config=[conf]    'Sets config file to load, values will be overriden by commandline args'
		                             -n --name   [name]    'Sets username, will prompt if not specified nor determined'
		                                --splash=[splash]  'Sets the amount of milliseconds the splash will be shown for (0 — off; default/invalid — 1000)'
		                             -t --poll-time [time] 'Sets the amount of time to wait between new message polls, in milliseconds (default/invalid — 3000)'
		                             -s --server [server]  'Sets the server to connect to'";

		let matches = Clapp::new("chattium-oxide-client").version(env!("CARGO_PKG_VERSION"))
		                                                 .author("nabijaczleweli <nabijaczleweli@gmail.com>")
		                                                 .about("Chat client for chattium-oxide-server")
		                                                 .args_from_usage(USAGE)
		                                                 .get_matches();
		let mut name  : Option<String> = None;
		let mut server: Option<String> = None;
		let mut splash: Option<String> = Some("1000".to_string());
		let mut time  : Option<String> = Some("3000".to_string());


		if let Some(config) = matches.value_of("config") {
			let mut yaml = YamlFileHandler::new();
			if yaml.add_files(vec![config]) {
				if let Some(yaml) = yaml.read_all_files().as_ref().map(|all| {
					let mut b = PathBuf::from(config);
					b.set_extension("");
					&all[b.file_name().unwrap().to_str().unwrap()]
				}) {
					name   = yaml["name"]  .as_str().map(|n| n.to_string());
					server = yaml["server"].as_str().map(|s| s.to_string());
					if let Some(spl) = yaml["splash"].as_i64().map(|s| s.to_string()) {
						splash = Some(spl);
					}
					if let Some(tme) = yaml["poll_time"].as_i64().map(|t| t.to_string()) {
						time = Some(tme);
					}
				}
			}
		}

		if let Some(cname)   = matches.value_of("name")      {if cname.len()   > 0 {name   = Some(cname.to_string())}}
		if let Some(cserver) = matches.value_of("server")    {if cserver.len() > 0 {server = Some(cserver.to_string())}}
		if let Some(csplash) = matches.value_of("splash")    {if csplash.len() > 0 {splash = Some(csplash.to_string())}}
		if let Some(ctime)   = matches.value_of("poll-time") {if ctime.len() > 0   {time   = Some(ctime.to_string())}}

		if name.is_none() {
			name = Some(match username() {
				Some(uname) =>
					if let Some("") = matches.value_of("name") {
						uname
					} else {
						terminal::print_xy(0, 0, &*&format!("Determined your username to {}.
If that's incorrect, type in your name now. Otherwise, hit <Return>: ", uname));
						terminal::refresh();
						match terminal::read_str(Point::new(0, 2), terminal::state::size().width).into_iter().flat_map(io::maybe_trimmed).next() {
							Some(rname) => rname,
							None        => uname,
						}
					},
				None => {
					let mut tname: Option<String> = None;
					while tname.is_none() {
						terminal::print_xy(0, 0, "No username specified and none could be determined.");
						let second_line = "Please type in your username now: ";
						terminal::print_xy(0, 1, second_line);
						terminal::refresh();
						for rname in terminal::read_str(Point::new(second_line.len() as i32, 1), terminal::state::size().width).into_iter().flat_map(io::maybe_trimmed) {
							tname = Some(rname);
						}
					}
					tname.unwrap()
				},
			});
			terminal::clear(None);
		}

		if server.is_some() {
			server = match validate_address(server.unwrap()) {
				Ok(sserver) => Some(sserver),
				Err(error)  => {
					terminal::print_xy(0, 3, &*&error);
					None
				},
			}
		}
		if server.is_none() {
			terminal::print_xy(0, 0, "No server specified.\nPlease type in the server address now: ");
			terminal::refresh();
			loop {
				if let Some(rserver) = terminal::read_str(Point::new(0, 2), terminal::state::size().width).into_iter().flat_map(io::maybe_trimmed).next() {
					match validate_address(rserver) {
						Ok(rserver) => {
							server = Some(rserver);
							break;
						},
						Err(error) => {
							terminal::clear(Some(Rect::from_size(Point::new(0, 3), terminal::state::size())));
							terminal::print_xy(0, 3, &*&error);
							terminal::refresh();
						},
					}
				}
			}
			terminal::clear(None);
			terminal::refresh();
		}

		assert!(name.is_some());
		assert!(server.is_some());
		assert!(splash.is_some());
		assert!(time.is_some());
		Options{
			name  : name.unwrap(),
			server: server.unwrap(),
			splash: max(i32::from_str(&*&splash.unwrap()).unwrap_or(1000), 0),
			time  : Duration::from_millis(u64::from_str(&*&time.unwrap()).unwrap_or(3000)),
		}
	}
}


/// Extract username from last segment of user's homedir
fn username() -> Option<String> {
	match home_dir() {
		Some(pbuf) =>
			match pbuf.as_path().file_name() {
				Some(fname) => fname.to_str().map(|string| string.to_string()),
				None => None,
			},
		None => None,
	}
}

fn validate_address(address: String) -> Result<String, String> {
	match (&*&address).to_socket_addrs() {
		Ok(_)      => Ok(address),
		Err(error) => Err(format!("{}", error)),
	}
}
