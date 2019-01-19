#[macro_use] extern crate honggfuzz;

extern crate blueprint;
use blueprint::{decode_to_json, decode_to_grid};

use std::ffi::CStr;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            //call export1
            let ret = decode_to_json(data.as_ptr(), data.len());
            let ret_array = unsafe {
                CStr::from_ptr(ret as *const i8).to_bytes()
            };
            //call export2
            let ret2 = decode_to_grid(
                ret_array.as_ptr(), 
                ret_array.len()
            );
            //let go of returned memory
            blueprint::free_return(ret);
            blueprint::free_return(ret2);            

        });
    }
}
