#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width*self.height
    }

    fn wider(&self,rect:&Rectangle) -> bool{
        self.width > rect.width
    }

    fn create(width:u32,height:u32) -> Rectangle{
        Rectangle { width, height }
    }
}

fn main(){
    let rect1 = Rectangle{width:30,height:50};
    println!("rect1 is {:?}",rect1);
    println!("rect1 is {:#?}",rect1);
    println!("rect1's area is {}",rect1.area());
    let rect2 = Rectangle{width:40,height:20};
    println!("rect1.width is {} greater than rect2.width.",rect1.wider(&rect2));

    let rect3 = Rectangle::create(30,50);
    println!("{:?}",rect3);

    struct UnitStruct;
}
