mod decode;
use decode::{b64_to_json, json_to_grid};
use std::ffi::CString;
#[no_mangle]
pub extern fn decode_to_json(s: *const u8, size: usize) -> *mut u8 {
	//takes base64 encoded blueprint and converts it to json
	//call the helper function (easy error passing)
	match b64_to_json(s, size) {
		Ok(e) => e,
		Err(e) => {
			println!("WARNING: {}", e);
			//default case
			CString::new("null").unwrap()
		}
	}.into_raw() as *mut u8
}

#[no_mangle]
pub extern fn decode_to_grid(s: *const u8, size: usize) -> *mut u8 {
	//takes json encoded blueprint and converts it to a grid
	// graphical representation of the blueprint
	//call the helper function (easy error passing)
	match json_to_grid(s, size) {
		Ok(e) => e,
		Err(e) => {
			println!("WARNING: {}", e);
			//default case
			CString::new("null").unwrap()
		}
	}.into_raw() as *mut u8
}

#[no_mangle]
pub extern fn free_return(s: *mut u8) {
	unsafe { CString::from_raw(s as *mut i8) };
}