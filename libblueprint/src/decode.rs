extern crate base64;
extern crate libflate;
extern crate json;

use self::libflate::zlib;
use std::io::Read;
use std::str;

use std::ffi::CString;
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

pub fn b64_to_json(s: *const u8, size: usize) -> Result<CString> {
	// interpret the input string
    if s.is_null() || size <= 0 {
        return Err(Error::NullArgument.into());
    }
    let r_str: &[u8] = unsafe { 
        std::slice::from_raw_parts(s, size)
    };
	// println!("DEBUG: Found string: {:?}", r_str);
	
	//decode the string. spec says to skip the first byte
	if r_str.len() <= 1 {
        //the string is too short to slice
		return Err(Error::StringTooShort.into());
	}
	let raw = base64::decode(&r_str[1..])?;

	//decompress
	let mut decoder = zlib::Decoder::new(&raw[..])?;
	let mut deflated = Vec::new();
	decoder.read_to_end(&mut deflated)?;

    //convert to json (to ensure validity)
    let parsed = json::parse(
        &str::from_utf8(&deflated)?
    )?.dump();

    //converting to cstring will error if there are null bytes
    return Ok(CString::new(parsed)?);

}

pub fn json_to_grid(s: *const u8, size: usize) -> Result<CString> {
    //interpret the input string
    if s.is_null() || size <= 0 {
        return Err(Error::NullArgument.into());
    }
    let r_str: &[u8] = unsafe { 
        std::slice::from_raw_parts(s, size)
    };
    //json-parse the data
    let parsed = json::parse(
        &str::from_utf8(&r_str)?
    )?;

    //converting to cstring will error if there are null bytes
    return Ok(CString::new(parsed.dump())?);

}