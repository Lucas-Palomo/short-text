use webpage::{WebpageOptions, Webpage};
use std::io::Error;

pub trait WebPageInterface {
    fn open_webpage(&self, uri: &str, option: Option<WebpageOptions>) -> Result<Webpage, Error>;

    fn read_webpage(&self, webpage: Webpage);
}
