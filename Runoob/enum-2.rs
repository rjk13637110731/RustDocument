#[derive(Debug)]

enum Book{
    Papery{index:u32},
    Electronic{url:String},
}

fn main(){
    let book = Book::Papery{index:1001};
    let ebook = Book::Electronic{url:String::from("url://...")};
    println!("{:?}",book);
    println!("{:?}",ebook);

    match book{
        Book::Papery { index } => {
            println!("Papery book {}",index);
        },
        Book::Electronic { url } => {
            println!("E-book {}",url);
        }
    }
}
