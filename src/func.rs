use crate::{data::{FileFormats, Error}, formats};
use std::{io::Read, collections::HashMap};

pub fn read_file_content(filename: &str) -> Option<String> {
	let mut file = std::fs::File::open(filename).expect("Something went wrong!");

	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Something went wrong!");

	Some(contents)
}

pub fn get_type_format(filename: &str) -> Result<FileFormats, Error> {
	let file_ending = filename.split_terminator('.').last().unwrap_or_default();

	match file_ending {
		"json" => Ok(FileFormats::Json),
		"toml" => Ok(FileFormats::Toml),
		"yaml" | "yml" => Ok(FileFormats::Yaml),
		"env" => Ok(FileFormats::Env),
		_ => Err(Error::UnsupportedFormat),
	}
}

pub fn read_configurations<T>(filename: &str) -> Result<T, Error>
where
	T: serde::de::DeserializeOwned + Default,
{
	let contents = read_file_content(filename);
	if contents.is_none() {
		return Err(Error::IOError);
	}
	let contents = contents.unwrap();

	let file_type = get_type_format(filename)?;

	let result = match file_type {
        FileFormats::Json => formats::json::deserialize::<T>(&contents)?,
        FileFormats::Toml => formats::toml::deserialize::<T>(&contents)?,
        FileFormats::Yaml => formats::yaml::deserialize::<T>(&contents)?,
		FileFormats::Env => {
			dotenv().ok();

			let mut a = HashMap::new();
			let vars = contents.split_once("=")
				.map(|(key, value)| a.insert(key, value));

			// dbg!(&vars);	

			let a = serde_json::to_string(&a).unwrap();
			dbg!(&a);

			// let contents =  serde_json::to_string(&content).unwrap();

			formats::json::deserialize::<T>(&a)?
		},
        _  => return Err(Error::UnsupportedFormat)
    };

	Ok(result)
}

macro_rules! deserializing {
	($id:ident) => {
		pub(crate) fn deserialize<T>(string_data: &str) -> Result<T, crate::data::Error>
		where
			T: serde::de::DeserializeOwned + Default,
		{
			if string_data.is_empty() {
				return Ok(T::default());
			}

			let new_data = $id::from_str(string_data);
			if new_data.is_err() {
				return Err(crate::data::Error::InvalidData);
			}
			let new_data = new_data.unwrap();

			Ok(new_data)
		}
	};
}

pub(crate) use deserializing;
use dotenv::dotenv;