use std::collections::HashMap;

fn main(){
    let mut s = String::from("run");
    s.push_str("oob");
    s.push('!');
    println!("{}",s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}",s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}",s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}",s1,s2,s3);
    println!("{}",s);

    let s = "hello world!";
    let len = s.len();
    println!("{}",len);

    let s = "Hello你好！";
    let len = s.len();
    println!("{}",len);

    let s = "Hello你好！";
    let len = s.chars().count();
    println!("{}",len);

    let s = String::from("Hello你好！");
    for c in s.chars() {
        println!("{}",c);
    }

    let s = String::from("Hello你好！");
    let a = s.chars().nth(2);
    println!("{:?}",a);

    let s = String::from("Hello你好！");
    let sub = &s[0..5];
    println!("{}",sub);

    let mut map = HashMap::new();
    map.insert("color", "red");
    map.insert("size", "10 m^2");
    println!("{}",map.get("color").unwrap());
    for p in map.iter() {
        println!("{:?}",p);
    }

    println!("Hello WORLD-----------------------");
    if let Some(x) = map.get_mut(&"color") {
        *x = "b";
    }
    for p in map.iter() {
        println!("{:?}",p);
    }
}
