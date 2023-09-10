fn main(){
    let mut v:Vec<i32> = vec![1,2,3,4];
    for i in &mut v{
        *i += 50;
        println!("{}",i);
    }
}
