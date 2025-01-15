use std::env::args;

use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
	let text = trpl::get(url).await.text().await;
	let title = Html::parse(&text)
		.select_first("title")
		.map(|title| title.inner_html());
	(url, title)
}

fn main() {
	let args: Vec<String> = args().collect();

	trpl::run(async {
		let url1 = page_title(&args[1]);
		let url2 = page_title(&args[2]);

		let (url, maybe_title) = match trpl::race(url1, url2).await {
			Either::Left(left) => left,
			Either::Right(right) => right,
		};

		println!("{} returned first", url);

		match maybe_title {
			Some(title) => println!("Its page title is: '{}'", title),
			None => println!("Its title could not be parsed."),
		}
	})
}
