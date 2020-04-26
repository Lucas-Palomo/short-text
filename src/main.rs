use short_text::{ShortText, WebPageInterface, FileInterface};

fn main() {

    let short = ShortText::new();
    let web = short.open_webpage("https://www.google.com.br", None);
    short.read_webpage(web.unwrap());

    let mut file = short.open_file("/home/darksrc/xbox.sh");

    short.read_file(file.unwrap());

}

