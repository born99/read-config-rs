pub enum FileFormats {
	Json,
	Yaml,
	Toml,
    Env
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("IOError.")]
    IOError,
    #[error("Submit unsupported format.")]
    UnsupportedFormat,
    #[error("Invalid data read.")]
    InvalidData
}