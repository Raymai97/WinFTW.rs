pub mod native;
pub mod hresult;
pub mod extend;

// they are all under 'ole'
pub use self::native::*;
pub use self::hresult::*;
// but these should be explicit
//pub use self::extend::*;
