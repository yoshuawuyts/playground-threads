use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();

  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move || {
    let val = String::from("hi");
    tx1.send(val).unwrap();
  });

  thread::spawn(move || {
    let val = String::from("sup");
    tx.send(val).unwrap();
  });

  for received in rx {
    println!("Got {}", received);
  }
}
