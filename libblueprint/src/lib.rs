mod decode;
use decode::b64_to_json;
use std::ffi::CString;
#[no_mangle]
pub extern fn decode_to_json(s: *const u8, size: usize) -> *mut u8 {
	//call the helper function (easy error passing)
	match b64_to_json(s, size) {
		Ok(e) => e,
		Err(e) => {
			println!("WARNING: {}", e);
			//default case
			CString::new("None").unwrap()
		}
	}.into_raw() as *mut u8
}
