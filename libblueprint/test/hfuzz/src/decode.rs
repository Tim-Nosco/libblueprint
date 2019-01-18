#[macro_use] extern crate honggfuzz;

extern crate blueprint;
use blueprint::decode;

use std::ffi::CString;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let ret = decode(data.as_ptr() as *const i8);
            unsafe{ CString::from_raw(ret); }
        });
    }
}
