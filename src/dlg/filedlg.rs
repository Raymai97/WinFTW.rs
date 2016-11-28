extern crate std;
extern crate winapi;
extern crate user32;

use self::winapi::HRESULT;
use err::WinftwErr;

pub struct FileDlg {
}

impl FileDlg {
	pub fn ask_for_file(&self) -> Result<Option<String>, WinftwErr> {
		let mut dt = MyData::new(MyMode::OpenFile);
		match my_show(&mut dt) {
			Err(x) => Err(x),
			_ => {
				if dt.result.len() == 0 { Ok(None) }
				else { Ok(Some(dt.result[0].clone())) }
			}
		}
	}

	pub fn ask_for_files(&self) -> Result<Option<Vec<String>>, WinftwErr> {
		let mut dt = MyData::new(MyMode::OpenFiles);
		match my_show(&mut dt) {
			Err(x) => Err(x),
			_ => {
				if dt.result.len() == 0 { Ok(None) }
				else { Ok(Some(dt.result)) }
			}
		}
	}

	pub fn ask_for_dir_path(&self) -> Result<Option<String>, WinftwErr> {
		let mut dt = MyData::new(MyMode::OpenDir);
		match my_show(&mut dt) {
			Err(x) => Err(x),
			_ => {
				if dt.result.len() == 0 { Ok(None) }
				else { Ok(Some(dt.result[0].clone())) }
			}
		}
	}

	pub fn ask_for_save_path(&self) -> Result<Option<String>, WinftwErr> {
		let mut dt = MyData::new(MyMode::SaveFile);
		match my_show(&mut dt) {
			Err(x) => Err(x),
			_ => {
				if dt.result.len() == 0 { Ok(None) }
				else { Ok(Some(dt.result[0].clone())) }
			}
		}
	}

}

enum MyMode {
	OpenFile,
	OpenFiles,
	OpenDir,
	SaveFile
}

struct MyData {
	mode: MyMode,
	result: Vec<String>
}

impl MyData {
	pub fn new(mode: MyMode) -> MyData {
		MyData { mode: mode, result: Vec::new() }
	}
}

fn my_show(dt: &mut MyData) -> Result<(), WinftwErr> {
	use self::winapi::*;
	use text::string_from_wide_null;
	use ole::*;
	
	let nullptr = std::ptr::null_mut();
	unsafe {
		let mut hr = CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);
		if hr.failed() { return Err(my_err("CoInitializeEx", hr)) }

		let mut pfd = nullptr as *mut IFileDialog;
		let clsid = match dt.mode {
			MyMode::SaveFile => CLSID_FileSaveDialog,
			_ => CLSID_FileOpenDialog
		};
		hr = CoCreateInstance(&clsid,
			nullptr as LPUNKNOWN,
			CLSCTX_INPROC_SERVER,
			&IID_IFileDialog,
			&mut pfd
				as *mut *mut IFileDialog
				as *mut _);
		if hr.failed() { return Err(my_err("CoCreateInstance", hr)) }

		let ref mut fd: IFileDialog = *pfd;
		if fd.Show(nullptr as HWND).succeeded() {
			match dt.mode {
				MyMode::OpenFile => {
					let mut psi = nullptr as *mut IShellItem;
					hr = fd.GetResult(&mut psi as *mut *mut IShellItem);
					if hr.failed() { return Err(my_err("GetResult", hr)) }

					let ref mut si: IShellItem = *psi;
					let mut wsz = nullptr as LPWSTR;
					hr = si.GetDisplayName(SIGDN_FILESYSPATH, &mut wsz);
					if hr.failed() { return Err(my_err("GetDisplayName", hr)) }
					
					dt.result.push(string_from_wide_null(wsz));
				},
				_ => {
					/* TODO */
				}
			}
		}
		fd.Release();
		CoUninitialize();
	}
	Ok(())
}

fn my_err(place: &'static str, hr: HRESULT) -> WinftwErr {
	WinftwErr {
		code: hr as i64,
		message: format!("Error at {}\n\n{}", place, hr.to_string())
	}
}
