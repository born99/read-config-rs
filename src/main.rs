use func::read_configurations;
use serde::{Deserialize, Serialize};

mod data;
mod func;
mod json;
mod toml;
mod yaml;

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

fn main() {
	// There not enough time for writing properly unit test
	// so i'm using this to debug as well as use the program functions.
	dbg!(read_configurations::<MyData>("test.toml"));

	/* PS:

	Hi Tushar,
	Actually I cannot come up with a right way to handle multiple formats at once automatically, so I
	go back and implement each file format by hands.

	There are 3 supported type at the moment: toml, yaml, and json.
	The program are also not in the right structure and there are no test modules and certain error handling.

	I'm a bit into thinking of a good way so that's all I can do at the moment due to time limits.
	But I think I can complete it properly tonight and send you another version on Monday.

	Thank you for your advice.
	*/
}
