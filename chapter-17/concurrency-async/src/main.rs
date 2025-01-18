use std::time::Duration;

fn main() {
	trpl::run(async {
		let future1 = async {
			for i in 1..10 {
				println!("Hi number {} from first task!", i);
				trpl::sleep(Duration::from_millis(500)).await;
			}
		};

		let future2 = async {
			for i in 1..5 {
				println!("Hi number {} from the second task!", i);
				trpl::sleep(Duration::from_millis(500)).await;
			}
		};

		trpl::join(future1, future2).await;
	});
}
