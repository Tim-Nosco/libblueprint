#[macro_use] extern crate honggfuzz;

extern crate blueprint;
use blueprint::decode_to_json;

use std::ffi::CStr;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let ret = decode_to_json(data.as_ptr(), data.len());
            unsafe{ CStr::from_ptr(ret as *mut i8); }
        });
    }
}
