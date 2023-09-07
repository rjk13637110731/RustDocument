fn main(){
    let num = 3;
    if num < 5{
        println!("条件为TRUE");
    }else{
        println!("条件为FALSE");
    }

    let a = 12;
    let b;
    if a > 0{
        b = 1;
    }else if a < 0{
        b = -1
    }else{
        b = 0;
    }
    println!("b is {}",b);

    let a = 3;
    let number = if a > 0 {1} else {-1};
    println!("number is {}",number);
}
