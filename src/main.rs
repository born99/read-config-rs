use dotenv::dotenv;
use func::read_configurations;
use serde::{Deserialize, Serialize};
use crate::func::read_file_content;

mod data;
mod func;
mod formats;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MyData {
	pub correct: Present,
	pub nearer: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Present {
	pub dot: f64,
	pub rubbed: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MyData2 {
	APP_ID: String
}

fn main() {
	

	// There not enough time for writing properly unit test
	// so i'm using this to debug as well as use the program functions.
	let a= read_configurations::<MyData2>("test.env").unwrap();

	dbg!(a);
	let b = dotenv::from_path("test.env").unwrap();
	dbg!(b);
	dbg!(dotenv::var("APP_ID"));
}
