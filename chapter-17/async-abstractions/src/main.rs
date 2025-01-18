use std::future::Future;
use std::time::Duration;

use trpl::Either;

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
	match trpl::race(future_to_try, trpl::sleep(max_time)).await {
		Either::Left(output) => Ok(output),
		Either::Right(_) => Err(max_time),
	}
}

fn main() {
	trpl::run(async {
		let slow = async {
			trpl::sleep(Duration::from_millis(100)).await;
			"I finished!"
		};

		match timeout(slow, Duration::from_millis(10)).await {
			Ok(message) => println!("Succeeded with '{}'", message),
			Err(duration) => {
				println!("Failed after {} seconds", duration.as_secs());
			}
		}
	})
}
