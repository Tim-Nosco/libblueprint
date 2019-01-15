//https://docs.rs/base64/0.10.0/base64/
//https://docs.rs/compress/0.1.2/compress/
//https://docs.rs/json/0.11.13/json/

//From: https://wiki.factorio.com/Blueprint_string_format
//A blueprint string is a JSON representation of the blueprint, compressed with zlib deflate and then encoded using base64 with a version byte in front. The version byte is currently 0 for vanilla 0.15 and 0.16. So to get the JSON representation of a blueprint from a blueprint string, skip the first byte, base64 decode the string, and finally decompress using zlib inflate.

use base64;
use libflate::zlib;
use json;
use std::io::Read;


#[no_mangle]
pub extern fn decode(_input:String)->String {
	//Skip the first byte, b64 decode
	println!("DEBUG: decoding input");
	let _raw = base64::decode(&_input[1..]).unwrap();
	//zlib decompress
	println!("DEBUG: initializing decoder");
	let mut decompressed = String::new();
	decompressed = match &mut zlib::Decoder::new(&*_raw) {
		Ok(decoder) => {
			match decoder.read_to_string(&mut decompressed) {
				Ok(_len) => decompressed,
				Err(error) => {
					println!("Unable to read data.\n{}", error);
					String::from("0")
				},
			}
		},
		Err(error) => {
			println!("Unable to initialize Decoder.\n{}", error);
			String::from("0")
		},
	};

	return decompressed;
}

#[no_mangle]
pub extern fn encode(_input:String)->String {
	return String::from("None");
}