extern crate libc;
extern crate base64;
extern crate libflate;

use libc::c_char;
use std::ffi::{CString,CStr};
use libflate::zlib;
use std::io::Read;


use std::fmt;  
use std::error::Error as StdError;
type Result<T> = std::result::Result<T, Box<StdError>>;

#[derive(Debug)]
pub enum Error {  
    NullArgument,
	StringTooShort,
}
impl fmt::Display for Error {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NullArgument => f.write_str("NullArgument"),
            Error::StringTooShort => f.write_str("StringTooShort"),
        }
    }
}
impl StdError for Error {  
    fn description(&self) -> &str {
        match *self {
            Error::NullArgument => "Null argument provided to function.",
            Error::StringTooShort => "The blueprint spec requires a string of length greater than 1.",
        }
    }
}

fn decode_helper(s: *const i8) -> Result<*mut c_char> {
	// interpret c string
    let c_str = unsafe {
        if s.is_null() {
			return Err(Error::NullArgument.into());
		}
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_bytes();
	println!("DEBUG: Found string: {:?}", r_str);
	//decode the string. spec says to skip the first byte
	if r_str.len() <= 1 {
		return Err(Error::StringTooShort.into());
	}
	let raw = base64::decode(&r_str[1..])?;
	//decompress
	let mut decoder = zlib::Decoder::new(&raw[..])?;
	let mut deflated = Vec::new();
	decoder.read_to_end(&mut deflated)?;
	let deflated = String::from_utf8(deflated)?;
	//print results
	println!("DEBUG: Decoded data: {:?}", deflated);
	let deflated = CString::new(deflated)?;
	return Ok(deflated.into_raw());
}

#[no_mangle]
pub extern fn decode(s: *const i8) -> *mut c_char {
	match decode_helper(s) {
		Ok(e) => e,
		Err(e) => {
			println!("WARNING: {}", e);
			CString::new("None").unwrap().into_raw()
		}
	}
}
