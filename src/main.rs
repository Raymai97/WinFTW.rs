#[macro_use]
extern crate winftw;

use winftw::dlg::msgbox;
use winftw::dlg::MsgboxIcon;
use winftw::dlg::MsgboxButton;
use winftw::dlg::FileDlg;

fn main() {
	msgbox!(Info => "Win32 starter pack for Rust", "winftw - Windows FTW!");
	let fd = FileDlg{};
	match fd.ask_for_file() {
		Err(x) => {
			msgbox!(Error => &format!("Error code: {}\n{}", x.code, x.message), "Error!");
		},
		Ok(x) => match x {
			None => {
				msgbox!(Info => "User selected nothing aka cancelled.", "Hmm");
			},
			Some(x) => {
				msgbox!(Info => &format!("User selected this: \n{}", x), "OK");
			}
		}
	}
}
