extern crate std;
extern crate winapi;
extern crate user32;

use self::winapi::HRESULT;
use self::winapi::WCHAR;
use err::WinftwErr;

struct NameSpec {
	pub name: &'static str,
	pub spec: &'static str
}

pub struct FileDlg {
	title: &'static str,
	namespecs: Vec<NameSpec>
}

impl FileDlg {
	pub fn new() -> FileDlg {
		FileDlg {
			title: "",
			namespecs: Vec::new()
		}
	}

	pub fn add_filter(&mut self, name: &'static str, spec: &'static str) {
		self.namespecs.push(
			NameSpec { name: name, spec: spec }
		);
	}

	pub fn set_title(&mut self, title: &'static str) {
		self.title = title;
	}

	pub fn ask_for_file(&self) -> Result<Option<String>, WinftwErr> {
		match MyFileDlg::new(MyMode::OpenFile, &self).show() {
			Err(x) => Err(x),
			Ok(paths) => {
				if paths.len() == 0 { Ok(None) }
				else { Ok(Some(paths[0].clone())) }
			}
		}
	}

	pub fn ask_for_files(&self) -> Result<Option<Vec<String>>, WinftwErr> {
		match MyFileDlg::new(MyMode::OpenFiles, &self).show() {
			Err(x) => Err(x),
			Ok(paths) => {
				if paths.len() == 0 { Ok(None) }
				else { Ok(Some(paths)) }
			}
		}
	}

	pub fn ask_for_dir_path(&self) -> Result<Option<String>, WinftwErr> {
		match MyFileDlg::new(MyMode::OpenDir, &self).show() {
			Err(x) => Err(x),
			Ok(paths) => {
				if paths.len() == 0 { Ok(None) }
				else { Ok(Some(paths[0].clone())) }
			}
		}
	}

	pub fn ask_for_save_path(&self) -> Result<Option<String>, WinftwErr> {
		match MyFileDlg::new(MyMode::SaveFile, &self).show() {
			Err(x) => Err(x),
			Ok(paths) => {
				if paths.len() == 0 { Ok(None) }
				else { Ok(Some(paths[0].clone())) }
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

struct MyNameSpec {
	pub name: Vec<WCHAR>,
	pub spec: Vec<WCHAR>
}

struct MyFileDlg {
	mode: MyMode,
	title: Vec<WCHAR>,
	rns_vec: Vec<MyNameSpec>
}

impl MyFileDlg {
	pub fn new(mode: MyMode, fd: &FileDlg) -> MyFileDlg {
		use text::ToWide;

		let mut dt = MyFileDlg {
			mode: mode,
			title: fd.title.to_wide_null(),
			rns_vec: Vec::new()
		};
		for ns in &fd.namespecs {
			dt.rns_vec.push( MyNameSpec {
				name: ns.name.to_wide_null(),
				spec: ns.spec.to_wide_null()
			});
		}
		dt
	}

	pub fn show(&self) -> Result<Vec<String>, WinftwErr> {
		use self::winapi::*;
		use ole::native::*;
		use ole::hresult::OkNotOk;
		use text::string_from_wide_null;
		use text::ToWide;

		let mut selected_paths: Vec<String> = Vec::new();
		let nullptr = std::ptr::null_mut();
		unsafe {
			let mut hr = CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);
			if hr.failed() { return Err(my_err("CoInitializeEx", hr)) }
			let mut pfd = nullptr as *mut c_void;
			let clsid = match self.mode {
				MyMode::SaveFile => CLSID_FileSaveDialog,
				_ => CLSID_FileOpenDialog
			};
			let iid = match self.mode {
				MyMode::SaveFile => IID_IFileDialog,
				_ => IID_IFileOpenDialog
			};
			hr = CoCreateInstance(&clsid,
				nullptr as LPUNKNOWN,
				CLSCTX_INPROC_SERVER,
				&iid,
				&mut pfd as *mut *mut c_void);
			if hr.failed() { return Err(my_err("CoCreateInstance", hr)) }
			let pfd = pfd as *mut IFileDialog;
			let ref mut fd = *pfd;
			// Set file dialog options
			let fd_options = match self.mode {
				MyMode::OpenFiles => { FOS_FILEMUSTEXIST | FOS_ALLOWMULTISELECT },
				MyMode::SaveFile => { FOS_OVERWRITEPROMPT },
				MyMode::OpenDir => { FOS_PICKFOLDERS }
				_ => { FOS_FILEMUSTEXIST }
			};
			hr = fd.SetOptions(fd_options);
			if hr.failed() { return Err(my_err("fd.SetOptions", hr)) }
			// Set file type filters
			let mut rg_spec: Vec<COMDLG_FILTERSPEC> = Vec::new();
			for rns in &self.rns_vec {
				rg_spec.push(COMDLG_FILTERSPEC {
					pszName: rns.name.as_ptr(),
					pszSpec: rns.spec.as_ptr()
				});
			}
			hr = fd.SetFileTypes(rg_spec.len() as DWORD, rg_spec.as_ptr());
			if hr.failed() { return Err(my_err("fd.SetFileTypes", hr)) }
			// HACK: Auto append file ext
			hr = fd.SetDefaultExtension("".to_wide_null().as_ptr());
			if hr.failed() { return Err(my_err("fd.SetDefaultExtension", hr)) }
			// Set file dialog title
			if self.title.len() > 0 {
				hr = fd.SetTitle(self.title.as_ptr());
				if hr.failed() { return Err(my_err("fd.SetTitle", hr)) }
			}
			// If user didn't cancel...
			if fd.Show(nullptr as HWND).succeeded() {
				match self.mode {
					MyMode::OpenFiles => {
						// Cast as IFileOpenDialog for GetResults()
						let pfd = pfd as *mut IFileOpenDialog;
						let ref mut fd = *pfd;
						let mut psia = nullptr as *mut IShellItemArray;					
						hr = fd.GetResults(&mut psia as *mut *mut IShellItemArray);
						if hr.failed() { return Err(my_err("fd.GetResults", hr)) }
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
							selected_paths.push(string_from_wide_null(wsz));
						}
					},
					_ => {
						let mut psi = nullptr as *mut IShellItem;
						hr = fd.GetResult(&mut psi as *mut *mut IShellItem);
						if hr.failed() { return Err(my_err("fd.GetResult", hr)) }
						let ref mut si: IShellItem = *psi;
						let mut wsz = nullptr as LPWSTR;
						hr = si.GetDisplayName(SIGDN_FILESYSPATH, &mut wsz);
						if hr.failed() { return Err(my_err("si.GetDisplayName", hr)) }
						selected_paths.push(string_from_wide_null(wsz));
					}
				}
			}
			fd.Release();
			CoUninitialize();
		}
		Ok(selected_paths)
	}
}

fn my_err(place: &'static str, hr: HRESULT) -> WinftwErr {
	use ole::hresult::OkNotOk;
	
	WinftwErr {
		code: hr as i64,
		message: format!("Error at {}\n\n{}", place, hr.to_string())
	}
}
