use std::io::prelude::*;
use std::fs::File;

fn main(){
    let mut file = File::create("\\teext.txt").unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}
