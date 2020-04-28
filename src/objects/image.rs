pub struct Image {
    url: String,
    description: String,
    title: String,
}

pub trait ImageOOPFunctions {
    fn set_url(&mut self, url: &str);
    fn set_description(&mut self, description: &str);
    fn set_title(&mut self, title: &str);

    fn get_url(&self) -> &String;
    fn get_description(&self) -> &String;
    fn get_title(&self) -> &String;
}

impl ImageOOPFunctions for Image {
    fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    fn get_url(&self) -> &String {
        return &self.url;
    }

    fn get_description(&self) -> &String {
        return &self.description;
    }

    fn get_title(&self) -> &String {
        return &self.title;
    }
}
