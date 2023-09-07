fn main(){
    let x = 5;
    let y = x;
    println!("x = {},y = {}",x,y);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {}",s2);

    let s1 = String::from("hello world");
    let s2 = s1.clone();
    println!("s1 = {},s2 = {}",s1,s2);

    let s = String::from("Hello");
    takes_owenrship(s);
    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}",s1);
    println!("{}",s3);

    let s1 = String::from("world");
    let s2 = &s1;
    println!("s1 is {},s2 is {}",s1,s2);

    let s1 = String::from("Hello World!");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.",s1,len);

    let s1 = String::from("hello world!!!!!!");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3;
    println!("s2 = {}",s2);

    let mut s1 = String::from("Hello World!");
    let s2 = &mut s1;
    s2.push_str("oonb");
    println!("s2 = {}",s2);
}

fn takes_owenrship(some_string:String){
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}",some_integer);
}

fn gives_ownership() ->String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}

fn calculate_length(s:&String) -> usize{
    s.len()
}
