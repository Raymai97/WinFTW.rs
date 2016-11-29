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

		let mut pfd = nullptr as *mut _;
		let clsid = match dt.mode {
			MyMode::SaveFile => CLSID_FileSaveDialog,
			_ => CLSID_FileOpenDialog
		};
		let iid = match dt.mode {
			MyMode::SaveFile => IID_IFileDialog,
			_ => IID_IFileOpenDialog
		};
		hr = CoCreateInstance(&clsid,
			nullptr as LPUNKNOWN,
			CLSCTX_INPROC_SERVER,
			&iid,
			&mut pfd
				as *mut *mut _);
		if hr.failed() { return Err(my_err("CoCreateInstance", hr)) }

		let pfd = pfd as *mut IFileDialog;
		let ref mut fd = *pfd;
		let mut fd_options: FILEOPENDIALOGOPTIONS = std::mem::uninitialized();
		hr = fd.GetOptions(&mut fd_options as *mut FILEOPENDIALOGOPTIONS);
		if hr.failed() { return Err(my_err("fd.GetOptions", hr)) }
		/*fd_options |= match dt.mode {
			MyMode::OpenFiles => FOS_ALLOWMULTISELECT,
			MyMode::OpenDir => FOS_PICKFOLDERS | FOS_FORCEFILESYSTEM,
			_ => 0
		};*/
		fd_options = match dt.mode {
			MyMode::OpenFiles => { FOS_FILEMUSTEXIST | FOS_ALLOWMULTISELECT },
			MyMode::SaveFile => { FOS_OVERWRITEPROMPT },
			MyMode::OpenDir => { FOS_PICKFOLDERS }
			_ => { FOS_FILEMUSTEXIST }
		};
		hr = fd.SetOptions(fd_options);
		if hr.failed() { return Err(my_err("fd.SetOptions", hr)) }
		if fd.Show(nullptr as HWND).succeeded() {
			match dt.mode {
				MyMode::OpenFiles => {
					/* Cast as we need IFileOpenDialog feature */
					let pfd = pfd as *mut IFileOpenDialog;
					let ref mut fd = *pfd;

					let mut psia = nullptr as *mut IShellItemArray;					
					hr = fd.GetResults(&mut psia as *mut *mut IShellItemArray);
					if hr.failed() { return Err(my_err("fd.GetResults", hr)) }

					// HACK: WinAPI doesn't support IShellItemArray yet, so...
					let psia = psia as *mut extend::_IShellItemArray;
					let ref mut sia = *psia;
					let mut count: DWORD = 0;
					hr = sia.GetCount(&mut count);
					if hr.failed() { return Err(my_err("sia.GetCount", hr)) }
					for i in 0..count {
						let mut psi = nullptr as *mut IShellItem;
						hr = sia.GetItemAt(i, &mut psi as *mut *mut IShellItem);
						if hr.failed() { return Err(my_err("sia.GetItemAt", hr)) }

						let ref mut si: IShellItem = *psi;
						let mut wsz = nullptr as LPWSTR;
						hr = si.GetDisplayName(SIGDN_FILESYSPATH, &mut wsz);
						if hr.failed() { return Err(my_err("si.GetDisplayName", hr)) }
						
						dt.result.push(string_from_wide_null(wsz));
					}
				},
				_ => {
					let mut psi = nullptr as *mut IShellItem;
					hr = fd.GetResult(&mut psi as *mut *mut IShellItem);
					if hr.failed() { return Err(my_err("GetResult", hr)) }

					let ref mut si: IShellItem = *psi;
					let mut wsz = nullptr as LPWSTR;
					hr = si.GetDisplayName(SIGDN_FILESYSPATH, &mut wsz);
					if hr.failed() { return Err(my_err("GetDisplayName", hr)) }
					
					dt.result.push(string_from_wide_null(wsz));
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
