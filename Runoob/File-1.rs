use std::fs::File;

fn main(){
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully!");
    }else{
        println!("Failed to open the file.");
    }
}
