fn f(i:i32) -> Result<i32,bool> {
    if i >= 0{Ok(i)}
    else{ Err(false)}
}


fn g(i:i32) -> Result<i32,bool>{
    let t = f(i)?;
    Ok(t)
}

fn main(){
    let r = g(10000);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}",v);
    }else{
        println!("Err");
    }
}
