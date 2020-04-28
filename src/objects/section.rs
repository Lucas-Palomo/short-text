use crate::objects::paragraph::Paragraph;

pub struct Section {
    title: String,
    paragraphs: Vec<Paragraph>,
}

trait SectionOOPFunctions {

    fn set_title(&mut self, title: &str);
    fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>);

    fn get_title(self) -> String;
    fn get_paragraphs(self) -> Vec<Paragraph>;

}

impl SectionOOPFunctions for Section {
    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs;
    }

    fn get_title(self) -> String {
        return self.title;
    }

    fn get_paragraphs(self) -> Vec<Paragraph> {
        return self.paragraphs;
    }
}

impl Default for Section {
    fn default() -> Self {
        return Section {
            title: String::new(),
            paragraphs: Vec::<Paragraph>::new(),
        };
    }
}
