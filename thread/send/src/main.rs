use core::time;
use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("1"),
            String::from("1"),
            String::from("1"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        tx1.send(String::from("hehe")).unwrap();
    });
    for res in rx {
        println!("Got: {}",res)
    }
}
