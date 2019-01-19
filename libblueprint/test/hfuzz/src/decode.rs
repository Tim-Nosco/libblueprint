#[macro_use] extern crate honggfuzz;

extern crate blueprint;
use blueprint::{decode_to_json, decode_to_grid};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            //call export1
            let ret = decode_to_json(data.as_ptr(), data.len());

            //call export2
            let ret2 = decode_to_grid(data.as_ptr(), data.len());

            //let go of returned memory
            blueprint::free_return(ret);
            blueprint::free_return(ret2);            

        });
    }
}
