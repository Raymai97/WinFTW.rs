extern crate std;
extern crate winapi;
extern crate user32;

use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use self::winapi::WCHAR;
use self::winapi::LPCWSTR;

pub trait ToWide {
	fn to_wide(&self) -> Vec<WCHAR>;
	fn to_wide_null(&self) -> Vec<WCHAR>;
}
impl<T> ToWide for T where T: AsRef<OsStr> {
	fn to_wide(&self) -> Vec<WCHAR> {
		self.as_ref().encode_wide().collect()
	}
	fn to_wide_null(&self) -> Vec<WCHAR> {
		self.as_ref().encode_wide().chain(Some(0)).collect()
	}
}

pub fn string_from_wide_null(ptr: LPCWSTR) -> String {
	unsafe {
		assert!(!ptr.is_null());
		let len = (0..std::isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap();
		let slice = std::slice::from_raw_parts(ptr, len);
		OsString::from_wide(slice).to_string_lossy().into_owned()
	}
}
