//https://docs.rs/base64/0.10.0/base64/
//https://docs.rs/compress/0.1.2/compress/
//https://docs.rs/json/0.11.13/json/

//From: https://wiki.factorio.com/Blueprint_string_format
//A blueprint string is a JSON representation of the blueprint, compressed with zlib deflate and then encoded using base64 with a version byte in front. The version byte is currently 0 for vanilla 0.15 and 0.16. So to get the JSON representation of a blueprint from a blueprint string, skip the first byte, base64 decode the string, and finally decompress using zlib inflate.

use base64;
use compress::zlib;
use json;

#[no_mangle]
pub extern fn decode(_input:String)->String {
	//Skip the first byte 
	let _raw = base64::decode(&_input[1..]).unwrap();
	// let _dec = 
	return String::from("0");
}

#[no_mangle]
pub extern fn encode(_input:String)->String {
	return String::from("None");
}