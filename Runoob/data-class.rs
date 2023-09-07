fn main(){
    let x = 2.0;
    let y : f32 = 4.0;
    println!("x = {},y = {}",x,y);

    let sum = 5+10;
    let difference = 95.5-4.3;
    let product = 4*30;
    let quotient = 56.7/32.2;
    let remainder = 43%5;
    println!("sum = {},difference = {},product = {},quotient = {},remainder = {}",sum,difference,product,quotient,remainder);

    let tup : (i32,f64,u8) = (500,6.4,1);
    let (x,y,z) = tup;
    
    let a = [1,2,3,4,5];
    let b = ["January","February","March"];
    let c: [i32; 5] = [1,2,3,4,5];
    let d = [3;5];
    let first = a[0];
    let y = a[1];
    let mut a = [1,2,3];
    a[0] = 4;
    println!("a[0] = {}",a[0]);
}
