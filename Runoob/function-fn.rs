fn main(){
    println!("Hello World!!!!");
    another_function();
    other_function(5,6);

    let x = 5;
    let y = {
        let x = 3;
        x+1
    };
    println!("x的值为：{}",x);
    println!("y的值为：{}",y);

    fn five() -> i32 {
        5
    }
    println!("five()的值为：{}",five());

    let sum = add(3,4);
    println!("sum = {}",sum);
}

fn another_function(){
    println!("Hello World!");
}

fn other_function(x:i32,y:i32){
    println!("x的值为：{}",x);
    println!("y的值为：{}",y);
}

fn add(a:i32,b:i32) -> i32{
    return a+b;
}
