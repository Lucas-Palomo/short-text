use crate::objects::section::Section;

pub struct Content {
    title: String,
    description: String,
    thumbnail: String,
    sections: Vec<Section>,
}

trait ContentOOPFunctions {
    fn set_title(&mut self, title: &str);
    fn set_description(&mut self, description: &str);
    fn set_thumbnail(&mut self, thumbnail: &str);
    fn set_sections(&mut self, sections: Vec<Section>);

    fn get_title(self) -> String;
    fn get_description(self) -> String;
    fn get_thumbnail(self) -> String;
    fn get_sections(self) -> Vec<Section>;
}

impl ContentOOPFunctions for Content {
    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    fn set_thumbnail(&mut self, thumbnail: &str) {
        self.thumbnail = thumbnail.to_string();
    }

    fn set_sections(&mut self, sections: Vec<Section>) {
        self.sections = sections;
    }

    fn get_title(self) -> String {
        return self.title;
    }

    fn get_description(self) -> String {
        return self.description;
    }

    fn get_thumbnail(self) -> String {
        return self.thumbnail;
    }

    fn get_sections(self) -> Vec<Section> {
        return self.sections;
    }
}

impl Default for Content {
    fn default() -> Self {
        return Content {
            title: String::new(),
            description: String::new(),
            thumbnail: String::new(),
            sections: Vec::<Section>::new()
        };
    }
}


