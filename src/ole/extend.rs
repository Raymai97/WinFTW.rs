// WinAPI v0.2.8 doesn't everything I needed, so...
// Just some terrible workaround...

#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate std;
extern crate winapi;
extern crate user32;

use self::winapi::*;

// Copied from winapi::macros
macro_rules! RIDL {
    (interface $interface:ident ($vtbl:ident)
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr),+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
    };
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {
    }) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $pvtbl
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl std::ops::Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }

    };
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident)
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl ::std::ops::Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };
}

RIDL!(
interface _IUnknown(_IUnknownVtbl) {
    fn QueryInterface(&mut self, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT,
    fn AddRef(&mut self) -> ULONG,
    fn Release(&mut self) -> ULONG
}
);

// TODO: Remove this when WinAPI crate support this
RIDL!(
interface _IShellItemArray(_IShellItemArrayVtbl): _IUnknown(_IUnknownVtbl) {
	fn BindToHandler(&mut self /* TODO */) -> HRESULT,
	fn GetPropertyStore(&mut self /* TODO */) -> HRESULT,
	fn GetPropertyDescriptionList(&mut self /* TODO */) -> HRESULT,
	fn GetAttributes(&mut self /* TODO */) -> HRESULT,
	fn GetCount(&mut self, pdwNumItems: LPDWORD) -> HRESULT,
	fn GetItemAt(&mut self, dwIndex: DWORD, ppsi: *mut *mut IShellItem) -> HRESULT,
	fn EnumItems(&mut self /* TODO */) -> HRESULT
}
);

