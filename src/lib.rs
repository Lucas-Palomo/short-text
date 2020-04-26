pub use crate::interfaces::file::FileInterface;
pub use crate::interfaces::webpage::WebPageInterface;
use webpage::{Webpage, WebpageOptions};
use std::io::{Error, Read};
use std::fs::{File, read_to_string};

pub mod interfaces;

pub struct ShortText {
}

impl ShortText {

    pub fn new() -> Self {
        return ShortText {}
    }

}

impl WebPageInterface for ShortText {
    fn open_webpage(&self, uri: &str, option: Option<WebpageOptions>) -> Result<Webpage, Error> {
        return Webpage::from_url(uri, option.unwrap_or(WebpageOptions::default()));
    }

    fn read_webpage(&self, webpage: Webpage) {
        println!("{}", webpage.http.body);
    }
}

impl FileInterface for ShortText {
    fn open_file(&self, uri: &str) -> Result<String, Error> {
        return read_to_string(uri);
    }

    fn read_file(&self, file_content: String) {
        println!("\n--\n{}", file_content)
    }
}
