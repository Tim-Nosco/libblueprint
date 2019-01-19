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
    JsonParseError,
}
impl fmt::Display for Error {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NullArgument => f.write_str("NullArgument"),
            Error::StringTooShort => f.write_str("StringTooShort"),
            Error::JsonParseError => f.write_str("JsonParseError"),
        }
    }
}
impl StdError for Error {  
    fn description(&self) -> &str {
        match *self {
            Error::NullArgument => "Null argument provided to function.",
            Error::StringTooShort => "The blueprint spec requires a string of length greater than 1.",
            Error::JsonParseError => "Incorrectly formatted blueprint json.",
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
    //take in some blueprint json string and create a 
    // new json format that contains an array of integers
    // representing the unique objects in the blueprint
    /* example grid:
    {   "key": [
            //idx: 0
            {"name": "assembler-2",
                "recipe": "copper-cable"},
            //idx: 1
            {"name": "fast-inserter",
                "direction": "left"},
            //idx: 2
            {"name": "fast-belt",
                "direction": "up"},
            //idx: 3
            {"name": "fast-inserter",
                "direction": "right"}
        ]
        "grid": [
            [ 0, 0, 0, 3, 2],
            [ 0, 0, 0,-1, 2],
            [ 0, 0, 0, 1, 2]
        ]
    }
    */

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
    //begin extracting data
    let entity_arr = &parsed["blueprint"]["entities"];
    println!("DEBUG: blueprint entities: {:#}", entity_arr);
    //determine min and max for x and y to calc board size
    println!("DEBUG: len: {}", entity_arr.len());
    let mut minx: i32 = 0; let mut miny: i32 = 0;
    let mut maxx: i32 = 0; let mut maxy: i32 = 0;
    for i in 0..entity_arr.len() {
        //TODO: wierd bug where splitters are 1/2 coordinate
        //TODO: account for central position coord when 
        // determining min and max position
        println!("DEBUG: i: {} {:#}", i, entity_arr[i]);
        let pos = &entity_arr[i]["position"];
        let x = match pos["x"].as_i32() {
            Some(e) => e,
            None => return Err(Error::JsonParseError.into())
        };
        let y = match pos["y"].as_i32() {
            Some(e) => e,
            None => return Err(Error::JsonParseError.into())
        };
        println!("DEBUG: x: {:#}, y: {:#}", x, y);
        if x < minx { minx = x; }
        if x > maxx { maxx = x; }
        if y < miny { miny = y; }
        if y > maxy { maxy = y; }
    }
    println!("DEBUG: minx {} maxx {} miny {} maxy {}", minx, maxx, miny, maxy);

    //converting to cstring will error if there are null bytes
    return Ok(CString::new(parsed.dump())?);

}