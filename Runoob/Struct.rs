struct Site{
    domain:String,
    name:String,
    nation:String,
    found:u32
}
fn main(){
    let runoob = Site{
        domian:String::from("www.runoob"),
        name:String::from("RUNOOB"),
        nation:String::from("China"),
        found:2013
    };

    let domain= String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let nation = String::from("China");
    let runoobb = Site{
        domian,
        name,
        nation,
        traffic:2013
    };

    let runoob = Site{
        domian:String::from("www.runoob"),
        name:String::from("RUNOOB"),
        ..runoob
    };


    struct Color(u8,u8,u8);
    struct Point(f64,f64);
    let black = Color(0,0,0);
    let origin = Point(0.0,0.0);
    println!("black = ({},{},{})",black.0,black.1,black.2);
    println!("origin = ({},{})",origin.0,origin.1);
}
