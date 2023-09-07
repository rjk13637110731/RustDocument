fn main(){
    let s = String::from("broadcast");
    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{} = {} + {}",s,part1,part2);

    let s1 = String::from("hello");
    let s2 = &s1[..];
    println!("s2 = {}",s2);

    let arr = [1,3,5,7,9];
    let part = &arr[0..3];
    for i in part.iter(){
        println!("{}",i);
    }
}
