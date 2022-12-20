#[allow(unused_imports)]

use std::fs::File ;
use std::io::Write;


fn main() {
    let  maybe_file = File::create("sample.txt");
    let mut file = match maybe_file {
        Ok(f) => f,
        Err(e) => panic!("unable to create a file {:?}", e ),
    };
   file.write_all("afshin created this file".as_bytes()).expect("can not write to file");
}