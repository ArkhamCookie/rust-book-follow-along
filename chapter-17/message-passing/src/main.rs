use std::time::Duration;

fn main() {
	trpl::run(async {
		let (transmitter, mut receiver) = trpl::channel();
		let transmitter1 = transmitter.clone();

		let transmitter1_future = async move {
			let values = vec![
				String::from("hi"),
				String::from("from"),
				String::from("the"),
				String::from("future"),
			];

			for value in values {
				transmitter1.send(value).unwrap();
				trpl::sleep(Duration::from_millis(500)).await;
			}
		};

			let transmitter_future = async move {
				let values = vec![
					String::from("more"),
					String::from("messages"),
					String::from("for"),
					String::from("you"),
				];

			for value in values {
				transmitter.send(value).unwrap();
				trpl::sleep(Duration::from_millis(1500)).await;
			}
		};

		let receiver_future = async {
			while let Some(value) = receiver.recv().await {
				println!("recieved: '{}'", value);
			}
		};

		trpl::join3(transmitter_future, transmitter1_future, receiver_future).await;
	});
}
