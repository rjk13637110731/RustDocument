fn main(){
    let mut number = 1;
    while number != 4{
        println!("{}",number);
        number += 1;
    }
    println!("EXIT");

    let mut i = 0;
    while i < 10{
        println!("i^2 = {}",i*i);
        i+=1;
    }
    println!("Exit");

    let a = [10,20,30,40,50];
    let mut j = 0;
    for i in a.iter(){
        j += 1;
        println!("a[{}] = {}",j,i);
    }
    println!("EXIT");

    let a = [10,20,30,40,50];
    for i in 0..5{
        println!("a[{}] = {}",i,a[i]);
    }
    println!("EXIT");

    let s = ['R','U','N','O','O','B'];
    let mut i = 0;
    loop{
        let ch = s[i];
        if ch == 'O'{
            break;
        }
        println!("\'{}\'",ch);
        i += 1;
    }
    println!("EXIT");
}
