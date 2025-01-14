use std::sync::mpsc;
use std::{thread, vec};
use std::time::Duration;

fn main() {
	let (sender, receviver) = mpsc::channel();
	let sender1 = sender.clone();

	thread::spawn(move || {
		let values = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];

		for value in values {
			sender1.send(value).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	thread::spawn(move || {
		let values = vec![
			String::from("more"),
			String::from("messages"),
			String::from("for"),
			String::from("you"),
		];

		for value in values {
			sender.send(value).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in receviver {
		println!("Got: {}", received);
	}
}
