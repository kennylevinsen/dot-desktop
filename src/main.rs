extern crate ini;
use ini::Ini;
use std::fs;
use std::io::ErrorKind;
use std::io::Error as io_error;
use std::error::Error;
use std::env;

#[derive(Debug)]
struct Desktop {
	name: String,
	exec: String,
	term: bool,
}

impl Desktop {
	fn parse(f: &str) -> Result<Desktop, Box<dyn Error>> {
    	let file = Ini::load_from_file(f)?;
    	match file.section(Some("Desktop Entry").to_owned()) {
	    	Some(desktop) => Ok(Desktop{
				name: desktop.get("Name").unwrap_or(&"".to_string()).to_string(),
				exec: desktop.get("Exec").unwrap_or(&"".to_string()).to_string(),
				term: desktop.get("Terminal").unwrap_or(&"".to_string()) == "true",
			}),
			None => Err(Box::new(io_error::new(ErrorKind::NotFound, "no desktop entry in file")))
    	}
	}

	fn parse_dir(d: &str) -> Result<Vec<Desktop>, Box<dyn Error>> {
		let mut files: Vec<Desktop> = Vec::new();
   		for entry in fs::read_dir(d)? {
   			let entry = entry?;
   			let path = entry.path();

   			if let Ok(d) = Desktop::parse(path.to_str().unwrap()) {
   				files.push(d)
   			}
   		}

   		Ok(files)
	}
}

fn main() {
	let home = env::var_os("HOME").unwrap().into_string().unwrap();
	let term = match env::var_os("DOTDESKTOP_TERM") {
		Some(s) => format!("{:} ", s.into_string().unwrap()).to_string(),
		None => "".to_string()
	};
	let app = match env::var_os("DOTDESKTOP_APP") {
		Some(s) => format!("{:} ", s.into_string().unwrap()).to_string(),
		None => "".to_string()
	};

	let paths: Vec<String> = vec![
		"/usr/share/applications".to_string(),
		"/usr/local/share/applications".to_string(),
		"/var/lib/flatpak/exports/share/applications".to_string(),
		format!("{}/.local/share/applications", home).to_string()];

	let desktop_entries: Vec<Desktop> = paths
		.into_iter()
		.map(|p| Desktop::parse_dir(&p))
		.filter_map(Result::ok)
		.flatten()
		.collect();

	let args: Vec<String> = env::args().collect();
	match args.len() {
		1 => {
			for d in desktop_entries {
				println!("{:}", d.name)
			}
		},
		2 => {
			for d in desktop_entries {
				if d.name == args[1] {
					println!("{:}{:}", match d.term {
						true => term,
						false => app,
					}, d.exec
						.replace("%f", "")
						.replace("%F", "")
						.replace("%u", "")
						.replace("%U", ""));
					return
				}
			}
		},
		_ => {
			eprintln!("dot-desktop takes 0 or 1 arguments");
			std::process::exit(1);
		}
	}
}
