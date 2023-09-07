#[derive(Debug)]

enum Option<T> {
    Some(T),
    None
}
fn main(){
    let opt = Option::Some("Hello");
    let opt:Option<&str> = Option::None;

    match opt {
        Option::Some(something) => {
            println!("{}",something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    let i = 0;
    match i {
        0 => println!("zero"),
        _ => {},
    }

    let i = 0;
    if let 0 = i {
        println!("zero");
    }

    enum Book{
        Papery(u32),
        Electronic(String)
    }
    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book{
        println!("Papery {}",index);
    }else{
        println!("Not papery book");
    }
}
