pub use crate::interfaces::file::FileInterface;
pub use crate::interfaces::webpage::WebPageInterface;
pub use crate::objects::content::Content;
use webpage::{Webpage, WebpageOptions};
use std::io::Error;
use std::fs::read_to_string;
use std::collections::HashMap;
use crate::objects::content::ContentFunctions;


pub mod objects;
pub mod interfaces;

pub struct ShortText {
    meta_extraction_strategy: MetaExtractionStrategy
}

impl ShortText {
    pub fn new() -> Self {
        return ShortText {
            meta_extraction_strategy: MetaExtractionStrategy::OpenGraph
        };
    }

    fn extract_meta_properties(&self, mut content: Content, meta: HashMap::<String, String>) -> Content {
        if !meta.is_empty() {
            use MetaExtractionStrategy::*;
            match self.get_meta_extraction_strategy() {
                Raw => {
                    content.set_title(meta.get("title").unwrap_or(&String::new()));
                    content.set_description(meta.get("description").unwrap_or(&String::new()));
                    content.set_thumbnail(meta.get("image").unwrap_or(&String::new()));
                }
                Twitter => {
                    content.set_title(meta.get("twitter:title").unwrap_or(&String::new()));
                    content.set_description(meta.get("twitter:description").unwrap_or(&String::new()));
                    content.set_thumbnail(meta.get("twitter:image").unwrap_or(&String::new()));
                }
                OpenGraph => {
                    content.set_title(meta.get("og:title").unwrap_or(&String::new()));
                    content.set_description(meta.get("og:description").unwrap_or(&String::new()));
                    content.set_thumbnail(meta.get("og:image").unwrap_or(&String::new()));
                }
            }
        }
        return content;
    }
}

pub trait ShortTextFunctions {
    fn set_meta_extraction_strategy(&mut self, strategy: MetaExtractionStrategy);
    fn get_meta_extraction_strategy(&self) -> &MetaExtractionStrategy;
}

impl ShortTextFunctions for ShortText {
    fn set_meta_extraction_strategy(&mut self, strategy: MetaExtractionStrategy) {
        self.meta_extraction_strategy = strategy;
    }

    fn get_meta_extraction_strategy(&self) -> &MetaExtractionStrategy {
        return &self.meta_extraction_strategy;
    }
}


impl WebPageInterface for ShortText {
    fn open_webpage(&self, uri: &str, option: Option<WebpageOptions>) -> Result<Webpage, Error> {
        return Webpage::from_url(uri, option.unwrap_or(WebpageOptions::default()));
    }

    fn map_webpage(&self, webpage: Webpage) -> Content {
        let mut content = Content::default();

        content = self.extract_meta_properties(content, webpage.html.meta);

        return content
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

pub enum MetaExtractionStrategy {
    Twitter,
    OpenGraph,
    Raw,
}
