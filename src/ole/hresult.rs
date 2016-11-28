extern crate std;
extern crate winapi;
//extern crate user32;

use self::winapi::HRESULT;

pub trait OkNotOk {
	fn succeeded(self) -> bool;
	fn failed(self) -> bool;
	fn to_string(self) -> String;
}

impl OkNotOk for HRESULT {
	fn succeeded(self) -> bool { self >= 0 }
	fn failed(self) -> bool { self < 0 }
	fn to_string(self) -> String { format!("HRESULT 0x{:08X}", self) }
}
