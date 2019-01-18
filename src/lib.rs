extern crate libc;

use libc::{c_char, uint32_t};
use std::ffi::{CString,CStr};
use base64;
use libflate::zlib;
use std::io::Read;

#[no_mangle]
pub extern fn decode(s: *const c_char) -> *mut c_char {
	//interpret c string
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
	//decode the string
	println!("DEBUG: Read string: {:?}", r_str);
	//spec says skip the first byte
	if r_str.chars().count() <= 1 {
		return CString::new("").unwrap().into_raw();
	}
	let raw = base64::decode(&r_str[1..]).unwrap();
	//decompress
	let mut decoder = zlib::Decoder::new(&raw[..]).unwrap();
	let mut deflated = Vec::new();
	decoder.read_to_end(&mut deflated).unwrap();
	let deflated = String::from_utf8(deflated).unwrap();
	//print results
	println!("DEBUG: Decoded data: {:?}", deflated);
	let deflated = CString::new(deflated).unwrap();
	deflated.into_raw()
}
