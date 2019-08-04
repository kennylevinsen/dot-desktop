extern crate ini;
use ini::Ini;
use std::fs;
use std::io::ErrorKind;
use std::io::Error as io_error;
use std::error::Error;
use std::env;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct Desktop {
	entry_type: String,
	name: String,
	no_display: bool,
	hidden: bool,
	exec: Option<String>,
	url: Option<String>,
	term: bool,
}

impl Desktop {
	fn parse(f: &str) -> Result<Desktop, Box<dyn Error>> {
		let file = Ini::load_from_file(f)?;
		match file.section(Some("Desktop Entry").to_owned()) {
			Some(desktop) => Ok(Desktop{
				entry_type: desktop.get("Type").unwrap_or(&"".to_string()).to_string(),
				name: desktop.get("Name").unwrap_or(&"".to_string()).to_string(),
				term: desktop.get("Terminal").unwrap_or(&"".to_string()) == "true",
				no_display: desktop.get("NoDisplay").unwrap_or(&"".to_string()) == "true",
				hidden: desktop.get("Hidden").unwrap_or(&"".to_string()) == "true",
				exec: desktop.get("Exec").map(|x| x.to_string()),
				url: desktop.get("URL").map(|x| x.to_string()),
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

impl Ord for Desktop {
	fn cmp(&self, other: &Self) -> Ordering { self.name.cmp(&other.name) }
}

impl PartialOrd for Desktop {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl PartialEq for Desktop {
	fn eq(&self, other: &Self) -> bool { self.name == other.name }
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
	let url = match env::var_os("DOTDESKTOP_URL") {
		Some(s) => format!("{:} ", s.into_string().unwrap()).to_string(),
		None => "xdg-open ".to_string()
	};

	let paths: Vec<String> = vec![
		"/usr/share/applications".to_string(),
		"/usr/local/share/applications".to_string(),
		"/var/lib/flatpak/exports/share/applications".to_string(),
		format!("{}/.local/share/applications", home).to_string(),
		format!("{}/.local/share/flatpak/exports/share/applications", home).to_string()];

	let mut desktop_entries: Vec<Desktop> = paths
		.into_iter()
		.map(|p| Desktop::parse_dir(&p))
		.filter_map(Result::ok)
		.flatten()
		.filter(|d| !d.hidden && !d.no_display && (d.entry_type == "Application" || d.entry_type == "Link"))
		.collect();

	desktop_entries.sort();

	let args: Vec<String> = env::args().collect();
	match args.len() {
		1 => {
			for d in desktop_entries {
				println!("{}", d.name)
			}
		},
		2|3 => {
			for d in desktop_entries {
				if d.name != args[1] {
					continue;
				}

				let arg = if args.len() == 3 {
					&args[2]
				} else {
					""
				};

				match d.entry_type.as_ref() {
					"Application" => {
						println!("{:}{:}", match d.term {
							true => term,
							false => app,
						}, d.exec.unwrap()
							.replace("%f", arg)
							.replace("%F", arg)
							.replace("%u", arg)
							.replace("%U", arg));
					}
					"Link" => println!("{:}{:}", url, d.url.unwrap()),
					_ => eprintln!("unsupported entry type: {:}", d.entry_type),
				};

				return;
			}
			eprintln!("no match for name");
			std::process::exit(2);
		},
		_ => {
			eprintln!("dot-desktop takes 0 or 1 arguments");
			std::process::exit(1);
		}
	}
}
