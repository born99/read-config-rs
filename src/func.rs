use crate::{data::FileFormats, json, toml, yaml};
use std::io::Read;

pub fn read_file_content(filename: &str) -> Option<String> {
	let mut file = std::fs::File::open(filename).expect("Something went wrong!");

	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Something went wrong!");

	Some(contents)
}

pub fn get_type_format(filename: &str) -> FileFormats {
	let file_ending = filename.split_terminator('.').last().unwrap_or_default();

	match file_ending {
		"json" => FileFormats::Json,
		"toml" => FileFormats::Toml,
		"yaml" | "yml" => FileFormats::Yaml,
		_ => panic!("Type of the file could not be inferred"),
	}
}

pub fn read_configurations<T>(filename: &str) -> Result<T, &'static str>
where
	T: serde::de::DeserializeOwned + Default,
{
	let contents = read_file_content(filename).expect("Cannot read file content!");
	let file_type = get_type_format(filename);

	let result = match file_type {
        FileFormats::Json => json::deserialize::<T>(&contents),
        FileFormats::Toml => toml::deserialize::<T>(&contents),
        FileFormats::Yaml => yaml::deserialize::<T>(&contents),
        _  => return Err("It is not supported at the moments. You can use other tools to convert file to supported formats to continue")
    };

	result
}

macro_rules! deserializing {
	($id:ident) => {
		pub use $id::Value;

		pub(crate) fn deserialize<T>(string_data: &str) -> Result<T, &'static str>
		where
			T: serde::de::DeserializeOwned + Default,
		{
			if string_data.is_empty() {
				return Ok(T::default());
			}

			let new_data = $id::from_str(string_data);
			if new_data.is_err() {
				return Err("Invalid data");
			}
			let new_data = new_data.unwrap();

			Ok(new_data)
		}
	};
}

pub(crate) use deserializing;
