extern crate std;
extern crate winapi;
extern crate user32;

use self::winapi::*;

#[allow(non_upper_case_globals)]
pub const CLSID_FileOpenDialog: CLSID = CLSID {
    Data1: 0xdc1c5a9c, Data2: 0xe88a, Data3: 0x4dde,
    Data4: [0xa5, 0xa1, 0x60, 0xf8, 0x2a, 0x20, 0xae, 0xf7]
};

#[allow(non_upper_case_globals)]
pub const CLSID_FileSaveDialog: CLSID = CLSID {
    Data1: 0xc0b4e2f3, Data2: 0xba21, Data3: 0x4773,
    Data4: [0x8d, 0xba, 0x33, 0x5e, 0xc9, 0x46, 0xeb, 0x8b]
};

#[allow(non_upper_case_globals)]
pub const IID_IFileDialog: IID = IID {
    Data1: 0x42f85136, Data2: 0xdb7e, Data3: 0x439c,
    Data4: [0x85, 0xf1, 0xe4, 0x07, 0x5d, 0x13, 0x5f, 0xc8]
};

#[link(name = "ole32")]
extern "system" {
	pub fn CoInitializeEx(
		pvReserved: LPVOID,
		dwCoInit: DWORD
	) -> HRESULT;
	pub fn CoUninitialize();
	pub fn CoCreateInstance(
        rclsid: REFCLSID,
        pUnkOuter: LPUNKNOWN,
        dwClsContext: DWORD,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
}
