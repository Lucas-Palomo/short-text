use std::fs::File;
use std::io::Error;

pub trait FileInterface {
    fn open_file(&self, uri: &str) -> Result<String, Error>;

    fn read_file(&self, file_content: String);
}
