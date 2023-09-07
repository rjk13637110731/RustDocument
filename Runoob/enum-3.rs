#[derive(Debug)]

enum Book{
    Papery(u32),
    Electronic{url:String},
}

fn main(){
    let book = Book::Papery(1001);
    let ebook = Book::Electronic{url:String::from("url://...")};
    println!("{:?}",book);
    println!("{:?}",ebook);

    match book{
        Book::Papery (i) => {
            println!("Papery book {}",i);
        },
        Book::Electronic { url } => {
            println!("E-book {}",url);
        }
    }
}
