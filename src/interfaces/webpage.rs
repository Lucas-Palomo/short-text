use webpage::{WebpageOptions, Webpage};
use std::io::Error;
use crate::Content;

pub trait WebPageInterface {
    fn open_webpage(&self, uri: &str, option: Option<WebpageOptions>) -> Result<Webpage, Error>;

    fn map_webpage(&self, webpage: Webpage) -> Content;

    // fn title_reasoner(&self, titles: Vec<>)
}
