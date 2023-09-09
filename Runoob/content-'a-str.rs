fn main(){
    struct Str<'a>{
        content:&'a str
    }

    let s = Str{
        content:"string_slice"
    };
    println!("s.content = {}",s.content);
}
