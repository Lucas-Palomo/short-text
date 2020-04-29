use crate::objects::image::Image;
use crate::objects::link::Link;
use crate::objects::list_element::ListElement;

pub struct Paragraph {
    paragraph: String,
    citation: String,
    images: Vec<Image>,
    links: Vec<Link>,
    list: Vec<ListElement>,
}

pub trait ParagraphFunctions {
    fn set_paragraph(&mut self, paragraph: &str);
    fn set_citation(&mut self, citation: &str);
    fn set_images(&mut self, images: Vec<Image>);
    fn set_links(&mut self, links: Vec<Link>);
    fn set_lists(&mut self, list: Vec<ListElement>);

    fn get_paragraph(&self) -> &String;
    fn get_citation(&self) -> &String;
    fn get_images(&self) -> &Vec<Image>;
    fn get_links(&self) -> &Vec<Link>;
    fn get_list(&self) -> &Vec<ListElement>;
}

impl ParagraphFunctions for Paragraph {
    fn set_paragraph(&mut self, paragraph: &str) {
        self.paragraph = paragraph.to_string();
    }

    fn set_citation(&mut self, citation: &str) {
        self.citation = citation.to_string();
    }

    fn set_images(&mut self, images: Vec<Image>) {
        self.images = images;
    }

    fn set_links(&mut self, links: Vec<Link>) {
        self.links = links;
    }

    fn set_lists(&mut self, list: Vec<ListElement>) {
        self.list = list;
    }

    fn get_paragraph(&self) -> &String {
        return &self.paragraph;
    }

    fn get_citation(&self) -> &String {
        return &self.citation;
    }

    fn get_images(&self) -> &Vec<Image> {
        return &self.images;
    }

    fn get_links(&self) -> &Vec<Link> {
        return &self.links;
    }

    fn get_list(&self) -> &Vec<ListElement> {
        return &self.list;
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        return Paragraph {
            paragraph: String::new(),
            citation: String::new(),
            images: Vec::<Image>::new(),
            links: Vec::<Link>::new(),
            list: Vec::<ListElement>::new(),
        };
    }
}
