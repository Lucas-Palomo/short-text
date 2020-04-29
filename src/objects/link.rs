pub struct Link {
    from: String,
    to: String,
    anchor: bool,
    text: String,
}

pub trait LinkFunctions {
    fn set_from(&mut self, from: &str);
    fn set_to(&mut self, to: &str);
    fn set_anchor(&mut self, anchor: bool);
    fn set_text(&mut self, text: &str);

    fn get_from(&self) -> &String;
    fn get_to(&self) -> &String;
    fn is_anchor(&self) -> &bool;
    fn get_text(&self) -> &String;
}

impl LinkFunctions for Link {
    fn set_from(&mut self, from: &str) {
        self.from = from.to_string();
    }

    fn set_to(&mut self, to: &str) {
        self.to = to.to_string();
    }

    fn set_anchor(&mut self, anchor: bool) {
        self.anchor = anchor;
    }

    fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    fn get_from(&self) -> &String {
        return &self.from;
    }

    fn get_to(&self) -> &String {
        return &self.to;
    }

    fn is_anchor(&self) -> &bool {
        return &self.anchor;
    }

    fn get_text(&self) -> &String {
        return &self.text;
    }
}
