fn main(){
    let mut a = 123;
    a = 456;
    const b : i32 = 145;
    println!("{}",a);
    println!("{}",b);

    let x = 5;
    let x = x + 5;
    let x = x * 5;
    println!("{}",x);

    // 有错误！
    let mut s = "abc";
    s = s.len();
    println!("{}",s);
}
