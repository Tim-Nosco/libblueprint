#[macro_use] extern crate honggfuzz;

extern crate blueprint;
use blueprint::decode;
use std::ffi::CString;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let s = unsafe {
                CString::from_vec_unchecked(data.to_vec()).into_raw()
            };
            decode(s);
        });
    }
}

