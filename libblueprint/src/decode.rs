extern crate base64;
extern crate libflate;
extern crate serde_json;

use self::libflate::zlib;
use std::io::Read;
use std::str;

use std::ffi::CString;
use std::fmt;  
use std::error::Error as StdError;
type Result<T> = std::result::Result<T, Box<StdError>>;

mod entities;
use self::entities::Entity;

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
    // println!("DEBUG: loading to json");
    let parsed: serde_json::Value = serde_json::from_slice(&deflated)?;
    //converting to cstring will error if there are null bytes
    // println!("DEBUG: loading to cstring");
    return Ok(CString::new(parsed.to_string())?);

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
    let parsed: serde_json::Value = serde_json::from_slice(&r_str)?;
    // //begin extracting data
    let entity_arr = &parsed["blueprint"]["entities"];
    println!("DEBUG: blueprint entities: {:#}", entity_arr);
    let entity_arr = match entity_arr.as_array() {
        Some(e) => e,
        None => return Err(Error::JsonParseError.into())
    };
    //determine min and max for x and y to calc board size
    println!("DEBUG: len: {}", entity_arr.len());
    let mut minx: i64 = 0; let mut miny: i64 = 0;
    let mut maxx: i64 = 0; let mut maxy: i64 = 0;
    for i in 0..entity_arr.len() {
        // println!("DEBUG: i: {} {:#}", i, entity_arr[i]);
        //parse the x,y from json
        let entity:Entity = serde_json::from_str(
            &entity_arr[i].to_string()
        )?;
        println!("DEBUG i: {} {:?}", i, entity);
        let x = entity.position.x.round() as i64;
        let y = entity.position.y.round() as i64;
        //TODO: account for central position coord when 
        // determining min and max position of assemblers.
        // Seems like +1 if pos or -1 if neg
        println!("DEBUG: x: {:#}, y: {:#}", x, y);
        //update boundries
        if x < minx { minx = x; }
        if x > maxx { maxx = x; }
        if y < miny { miny = y; }
        if y > maxy { maxy = y; }
        if entity.name.contains("assembling-machine") {
            if x-1 < minx { minx = x-1; }
            if x+1 > maxx { maxx = x+1; }
            if y-1 < miny { miny = y-1; }
            if y+1 > maxy { maxy = y+1; }
        }
    }
    let width = maxx-minx+1;
    let height = maxy-miny+1;
    println!("DEBUG: minx {} maxx {} miny {} maxy {}", minx, maxx, miny, maxy);
    println!("DEBUG: width: {} height: {}", width, height);

    //converting to cstring will error if there are null bytes
    return Ok(CString::new(parsed.to_string())?);

}