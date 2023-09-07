use std::fs::File;

fn main(){
    let f = File::open("hello.txt").unwrap();
    let f2 = File::open("helllo.txt").except("Failed to open.");
}
