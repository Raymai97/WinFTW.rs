extern crate std;
extern crate winapi;
extern crate user32;

use text::ToWide;
//use self::winapi::windef::HWND;

pub enum MsgboxIcon {
	None,
	Information,
	Question,
	Error,
	Warning
}

pub enum MsgboxButton {
	Ok,
	OkCancel,
	AbortRetryIgnore,
	YesNoCancel,
	YesNo,
	RetryCancel,
	CancelRetryContinue
}

pub fn msgbox(message: &str, title: &str, icon: MsgboxIcon, btn: MsgboxButton) {
	// to prevent vec ptr get freed too early
	let message_vec = message.to_wide_null();
	let title_vec = title.to_wide_null();
    unsafe {
    	let sz_message = message_vec.as_ptr();
    	let sz_title = title_vec.as_ptr();
		let dw_icon = match icon {
			MsgboxIcon::Information => winapi::MB_ICONASTERISK,
			MsgboxIcon::Question => winapi::MB_ICONQUESTION,
			MsgboxIcon::Error => winapi::MB_ICONHAND,
			MsgboxIcon::Warning => winapi::MB_ICONEXCLAMATION,
			_ => 0
		};
		let dw_btn = match btn {
			MsgboxButton::OkCancel => winapi::MB_OKCANCEL,
			MsgboxButton::AbortRetryIgnore => winapi::MB_ABORTRETRYIGNORE,
			MsgboxButton::YesNoCancel => winapi::MB_YESNOCANCEL,
			MsgboxButton::YesNo => winapi::MB_YESNO,
			MsgboxButton::RetryCancel => winapi::MB_RETRYCANCEL,
			MsgboxButton::CancelRetryContinue => winapi::MB_CANCELTRYCONTINUE,
			_ => 0
		};
        user32::MessageBoxW(std::ptr::null_mut(),
            sz_message, sz_title, dw_icon | dw_btn);
    }
}

#[macro_export]
macro_rules! msgbox {
	($msg: expr, $title: expr) => {
		msgbox!($msg, $title, MsgboxIcon::None)
	};
	(Info => $msg: expr, $title: expr) => {
		msgbox!($msg, $title, MsgboxIcon::Information)
	};
	(Warning => $msg: expr, $title: expr) => {
		msgbox!($msg, $title, MsgboxIcon::Warning)
	};
	(Error => $msg: expr, $title: expr) => {
		msgbox!($msg, $title, MsgboxIcon::Error)
	};
	(YesNo => $msg: expr, $title: expr) => {
		msgbox($msg, $title, MsgboxIcon::Question, MsgboxButton::YesNo)
	};
	($msg: expr, $title: expr, $icon: expr) => {
		msgbox($msg, $title, $icon, MsgboxButton::Ok)
	};
}
