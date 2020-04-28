pub use crate::interfaces::file::FileInterface;
pub use crate::interfaces::webpage::WebPageInterface;
pub use crate::objects::content::Content;
use webpage::{Webpage, WebpageOptions};
use std::io::{Error};
use std::fs::{read_to_string};
use std::collections::{HashMap};
use crate::objects::content::ContentOOPFunctions;


pub mod objects;
pub mod interfaces;

pub struct ShortText {}

impl ShortText {
    pub fn new() -> Self {
        return ShortText {};
    }

    pub fn analyze(&self, t: ReadType) {
        use ReadType::*;
        match t {
            Test => {
                println!("Test")
            }
            Frequency => {
                println!("F")
            }
        }
    }

    fn analyze_opengraph(&self, mut content: Content, opengraph: HashMap::<String, String>) -> Content {
        if !opengraph.is_empty() {
            // println!("{:#?}", opengraph);
            // content.set_title("title")
            // if opengraph.contains_key("title") {
            content.set_title(opengraph.get("title").unwrap_or(&String::new()));
            // }
            // if opengraph.contains_key("description") {
            content.set_description(opengraph.get("description").unwrap_or(&String::new()))
            // }
        }
        return content;
    }
}

impl WebPageInterface for ShortText {
    fn open_webpage(&self, uri: &str, option: Option<WebpageOptions>) -> Result<Webpage, Error> {
        return Webpage::from_url(uri, option.unwrap_or(WebpageOptions::default()));
    }

    fn map_webpage(&self, webpage: Webpage) {
        let mut content = Content::default();

        content = self.analyze_opengraph(content, webpage.html.opengraph.properties);

        println!("{}\n{}", content.get_title(), content.get_description())
        // content.set_title(webpage.html.opengraph.properties.get("title").unwrap());
        //
        // let tree_map = BTreeMap::<String, String>::new();
        // println!("{:#?}", webpage.html.meta);
        // println!("{:#?}", webpage.html.feed);
        // println!("{:#?}", webpage.html.schema_org);
        // println!("{:#?}", webpage.html.opengraph);
        // println!("{:#?}", webpage.html.description);
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
    Test,
}
