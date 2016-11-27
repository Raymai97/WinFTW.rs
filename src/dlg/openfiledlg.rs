extern crate std;
extern crate winapi;
extern crate user32;

use self::winapi::IFileDialog;

pub struct OpenFileDlg {
	pub filename: &'static str
}

impl OpenFileDlg {
	pub fn new() -> OpenFileDlg {
		OpenFileDlg {
			filename : ""
		}
	}
	pub fn show(&self) -> bool {
		let fd = IFileDialog::new();
		return false;
	}
}