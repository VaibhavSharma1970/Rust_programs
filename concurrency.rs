use std::thread;
use std::time::Duration;
fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hii number: {} from the spawn thread", i);
            thread::sleep(Duration::from_millis(1))
        }
    });
    for i in 1..5 {
        println!("Hii number: {} from the main thread", i);
        thread::sleep(Duration::from_millis(1))
    };
}
