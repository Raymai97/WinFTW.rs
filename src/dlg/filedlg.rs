extern crate std;
extern crate winapi;
extern crate user32;

use self::winapi::HRESULT;
use self::winapi::WCHAR;
use err::WinftwErr;

pub struct FileDlgNameSpec {
	pub name: &'static str,
	pub spec: &'static str
}

pub struct FileDlg {
	title: &'static str,
	namespecs: Vec<FileDlgNameSpec>
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
			FileDlgNameSpec { name: name, spec: spec }
		);
	}

	pub fn set_title(&mut self, title: &'static str) {
		self.title = title;
	}

	pub fn ask_for_file(&self) -> Result<Option<String>, WinftwErr> {
		let mut dt = MyData::new(MyMode::OpenFile, &self.title, &self.namespecs);
		match my_show(&mut dt) {
			Err(x) => Err(x),
			_ => {
				if dt.results.len() == 0 { Ok(None) }
				else { Ok(Some(dt.results[0].clone())) }
			}
		}
	}

	pub fn ask_for_files(&self) -> Result<Option<Vec<String>>, WinftwErr> {
		let mut dt = MyData::new(MyMode::OpenFiles, &self.title, &self.namespecs);
		match my_show(&mut dt) {
			Err(x) => Err(x),
			_ => {
				if dt.results.len() == 0 { Ok(None) }
				else { Ok(Some(dt.results)) }
			}
		}
	}

	pub fn ask_for_dir_path(&self) -> Result<Option<String>, WinftwErr> {
		let mut dt = MyData::new(MyMode::OpenDir, &self.title, &self.namespecs);
		match my_show(&mut dt) {
			Err(x) => Err(x),
			_ => {
				if dt.results.len() == 0 { Ok(None) }
				else { Ok(Some(dt.results[0].clone())) }
			}
		}
	}

	pub fn ask_for_save_path(&self) -> Result<Option<String>, WinftwErr> {
		let mut dt = MyData::new(MyMode::SaveFile, &self.title, &self.namespecs);
		match my_show(&mut dt) {
			Err(x) => Err(x),
			_ => {
				if dt.results.len() == 0 { Ok(None) }
				else { Ok(Some(dt.results[0].clone())) }
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

struct MyRawNameSpec {
	pub name: Vec<WCHAR>,
	pub spec: Vec<WCHAR>
}

struct MyData {
	mode: MyMode,
	title: Vec<WCHAR>,
	rns_vec: Vec<MyRawNameSpec>,
	results: Vec<String>
}

impl MyData {
	pub fn new(mode: MyMode, title: &'static str, namespecs: &Vec<FileDlgNameSpec>) -> MyData {
		use text::ToWide;

		let make_rns_vec = || -> Vec<MyRawNameSpec> {
			let mut v = Vec::with_capacity(namespecs.len());
			for ns in namespecs {
				v.push( MyRawNameSpec {
					name: ns.name.to_wide_null(),
					spec: ns.spec.to_wide_null()
				});
			} v
		};
		MyData {
			mode: mode,
			title: title.to_wide_null(),
			rns_vec: make_rns_vec(),
			results: Vec::new()
		}
	}
}

fn my_show(dt: &mut MyData) -> Result<(), WinftwErr> {
	use self::winapi::*;
	use ole::native::*;
	use ole::hresult::OkNotOk;
	use text::string_from_wide_null;
	use text::ToWide;

	let nullptr = std::ptr::null_mut();
	unsafe {
		let mut hr = CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);
		if hr.failed() { return Err(my_err("CoInitializeEx", hr)) }
		let mut pfd = nullptr as *mut c_void;
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
			&mut pfd as *mut *mut c_void);
		if hr.failed() { return Err(my_err("CoCreateInstance", hr)) }
		let pfd = pfd as *mut IFileDialog;
		let ref mut fd = *pfd;
		// Set file dialog options
		let fd_options = match dt.mode {
			MyMode::OpenFiles => { FOS_FILEMUSTEXIST | FOS_ALLOWMULTISELECT },
			MyMode::SaveFile => { FOS_OVERWRITEPROMPT },
			MyMode::OpenDir => { FOS_PICKFOLDERS }
			_ => { FOS_FILEMUSTEXIST }
		};
		hr = fd.SetOptions(fd_options);
		if hr.failed() { return Err(my_err("fd.SetOptions", hr)) }
		// Set file type filters
		let mut rg_spec: Vec<COMDLG_FILTERSPEC> = Vec::with_capacity(dt.rns_vec.len());
		for rns in &dt.rns_vec {
			rg_spec.push(COMDLG_FILTERSPEC {
				pszName: rns.name.as_ptr(),
				pszSpec: rns.spec.as_ptr()
			});
		}
		hr = fd.SetFileTypes(rg_spec.len() as DWORD, rg_spec.as_ptr());
		if hr.failed() { return Err(my_err("fd.SetFileTypes", hr)) }
		// HACK: Auto append file ext
		fd.SetDefaultExtension("".to_wide_null().as_ptr());
		// Set file dialog title
		if dt.title.len() > 0 {
			fd.SetTitle(dt.title.as_ptr());
		}
		// If user didn't cancel...
		if fd.Show(nullptr as HWND).succeeded() {
			match dt.mode {
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
						dt.results.push(string_from_wide_null(wsz));
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
					dt.results.push(string_from_wide_null(wsz));
				}
			}
		}
		fd.Release();
		CoUninitialize();
	}
	Ok(())
}

fn my_err(place: &'static str, hr: HRESULT) -> WinftwErr {
	use ole::hresult::OkNotOk;
	
	WinftwErr {
		code: hr as i64,
		message: format!("Error at {}\n\n{}", place, hr.to_string())
	}
}
