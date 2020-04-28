pub use crate::interfaces::file::FileInterface;
pub use crate::interfaces::webpage::WebPageInterface;
pub use crate::objects::content::Content;
use webpage::{Webpage, WebpageOptions};
use std::io::{Error, Read};
use std::fs::{File, read_to_string};
use std::collections::BTreeMap;


pub mod objects;
pub mod interfaces;

pub struct ShortText {
}

impl ShortText {

    pub fn new() -> Self {
        return ShortText {}
    }

    pub fn analyze(&self, t: ReadType) {
        use ReadType::*;
        match t {
            Test => {
                println!("Test")
            },
            Frequency => {
                println!("F")
            }
        }
    }

}

impl WebPageInterface for ShortText {
    fn open_webpage(&self, uri: &str, option: Option<WebpageOptions>) -> Result<Webpage, Error> {
        return Webpage::from_url(uri, option.unwrap_or(WebpageOptions::default()));
    }

    fn map_webpage(&self, webpage: Webpage) {
        let content = Content::default();

        content
        // let tree_map = BTreeMap::<String, String>::new();
        println!("{:#?}", webpage.html.meta);
        println!("{:#?}", webpage.html.feed);
        println!("{:#?}", webpage.html.schema_org);
        println!("{:#?}", webpage.html.opengraph);
        println!("{:#?}", webpage.html.description);
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

pub enum ReadType {
    // NgramUnion(u32),
    Frequency,
    Test
}
