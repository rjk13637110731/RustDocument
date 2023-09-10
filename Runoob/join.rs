use std::thread;
use std::time::Duration;

fn main(){
    thread::spawn(||{
        for i in 0..5 {
            println!("spawned thread print {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("spawned thread print {}",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
