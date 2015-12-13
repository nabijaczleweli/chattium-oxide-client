use clap::App as Clapp;
use std::io::{self, Write};
use std::env::home_dir;
use std::fmt::Arguments as FormatArguments;
use std::path::PathBuf;
use yaml_file_handler::yaml_handler::FileHandler as YamlFileHandler;


#[derive(Debug)]
pub struct Options {
	pub name: String,
	pub server: String,
}


impl Options {
	/// Parses commandline arguments into an [`Options`](#) instance
	///
	/// Optionally reads from a config file in [YAML](http://yaml.org) format, however commandline arguments take preference thereover.
	/// The config file format is trivial: all root keys and values are of the same name and format as long commandline arguments,
	/// see `"example/config.yml"`.
	pub fn parse() -> Options {
		const USAGE: &'static str = "-c --config=[conf]   'Sets config file to load, values will be overriden by commandline args'
		                             -n --name [name]     'Sets username, will prompt if not specified nor determined'
		                             -s --server [server] 'Sets the server to connect to'";

		let matches = Clapp::new("chattium-oxide-client").version("0.1.0")
		                                                 .author("nabijaczleweli <nabijaczleweli@gmail.com>")
		                                                 .about("Chat client for chattium-oxide-server")
		                                                 .args_from_usage(USAGE)
		                                                 .get_matches();
		let mut name: Option<String> = None;
		let mut server: Option<String> = None;
		if let Some(conf) = matches.value_of("conf") {
			let mut yaml = YamlFileHandler::new();
			if yaml.add_files(vec![conf]) {
				if let Some(yaml) = yaml.read_all_files().as_ref().map(|all| {
					let mut b = PathBuf::from(conf);
					b.set_extension("");
					&all[b.file_name().unwrap().to_str().unwrap()]
				}) {
					name = yaml["name"].as_str().map(|n| n.to_string());
					server = yaml["server"].as_str().map(|n| n.to_string());
				}
			}
		}

		if let Some(cname)   = matches.value_of("name")   {name   = Some(cname.to_string())}
		if let Some(cserver) = matches.value_of("server") {server = Some(cserver.to_string())}

		if name.is_none() {
			name = Some(match username() {
				Some(uname) =>
					match read_prompted(format_args!("Determined your username to {}.\nIf that's incorrect, type in your name now. Otherwise, hit <Return>: ", uname)) {
						Ok(rname) =>
							match rname {
								Some(rname) => rname,
								None => uname,
							},
						Err(_) => {
							println!("Failed to read custom name, assuming default OK.");
							uname
						}
					},
				None => {
					let mut tname: Option<String> = None;
					while tname.is_none() {
						match read_prompted(format_args!("No username specified and none could be determined.\nPlease type in your username now: ")) {
							Ok(rname) => tname = rname,
							Err(error) => println!("Couldn't read username: {}", error),
						}
					}
					tname.unwrap()
				},
			});
		}

		if server.is_none() {
			let mut tserver: Option<String> = None;
			while tserver.is_none() {
				match read_prompted(format_args!("No server specified.\nPlease type in the server address now: ")) {
					Ok(rserver) => tserver = rserver,
					Err(error) => println!("Couldn't read server: {}", error),
				}
			}
			server = tserver;
		}

		assert!(name.is_some());
		assert!(server.is_some());
		Options{
			name: name.unwrap(),
			server: server.unwrap(),
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

fn read_prompted(prompt: FormatArguments) -> io::Result<Option<String>> {
	try!(io::stdout().write_fmt(prompt));
	try!(io::stdout().flush());

	let mut obuf = String::new();
	try!(io::stdin().read_line(&mut obuf));  // Don't match on this directly, because it returns with "\r\n"
	let obuf = obuf.trim();
	Ok(match obuf.len() {
		0 => None,
		_ => Some(obuf.trim().to_string()),
	})
}
