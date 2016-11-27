#[macro_use]
extern crate winftw;

use winftw::dlg::msgbox;
use winftw::dlg::MsgboxIcon;
use winftw::dlg::MsgboxButton;
use winftw::dlg::OpenFileDlg;

fn main() {
	msgbox!(Info => "Win32 starter pack for Rust", "winftw - Windows FTW!");
	let ofd = OpenFileDlg::new();
	if ofd.show() {
		msgbox!(Info => &format!("You've selected {}. ", ofd.filename), "Omo");
	}
}
