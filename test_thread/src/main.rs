use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(||{
        for i in 0..10{
            println!("Hello number {i} from 子线程");
            thread::sleep(Duration::from_millis(1));
        }
    });
}
