use crate::objects::link::Link;

pub struct ListElement {
    link: Link,
    text: String,
}

pub trait ListElementOOPFunctions {
    fn set_link(&mut self, link: Link);
    fn set_text(&mut self, text: &str);

    fn get_link(&self) -> &Link;
    fn get_text(&self) -> &String;
}

impl ListElementOOPFunctions for ListElement {
    fn set_link(&mut self, link: Link) {
        self.link = link;
    }

    fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    fn get_link(&self) -> &Link {
        return &self.link;
    }

    fn get_text(&self) -> &String {
        return &self.text;
    }
}
