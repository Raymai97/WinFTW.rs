#[macro_use]
extern crate winftw;

use winftw::dlg::msgbox::*;
use winftw::dlg::filedlg::*;

fn main() {
	msgbox!(Info => "Win32 starter pack for Rust", "winftw - Windows FTW!");
	demo_open_files();
	demo_save_file();
}

fn demo_save_file() {
	let mut fd = FileDlg::new();
	fd.add_filter("Web Pages", "*.htm;*.html");
	fd.add_filter("Batch Files", "*.bat;*.cmd");
	fd.set_title("Just a demo, it would not save anything");
	match fd.ask_for_save_path() {
		Err(x) => {
			msgbox!(Error => &x.message, "Error!");
		},
		Ok(x) => match x {
			None => {
				msgbox!(Info => "User didn't save aka cancelled.", "Hmm");
			},
			Some(x) => {
				msgbox!(Info => &x, "Save path...");
			}
		}
	}
}

fn demo_open_files() {
	let mut fd = FileDlg::new();
	fd.add_filter("Audio files", "*.mp3;*.wma;*.wav");
	fd.add_filter("Text files", "*.txt");
	match fd.ask_for_files() {
		Err(x) => {
			msgbox!(Error => &x.message, "Error!");
		},
		Ok(x) => match x {
			None => {
				msgbox!(Info => "User selected nothing aka cancelled.", "Hmm");
			},
			Some(files) => {
				let mut msg = "User have selected: ".to_string();
				for file in files {
					msg += &format!("\n{}", file);
				}
				msgbox!(Info => &msg, "Selected files");
			}
		}
	}
}
