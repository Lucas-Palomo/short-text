use short_text::{ShortText, WebPageInterface, FileInterface, ReadType};

fn main() {

    let short = ShortText::new();
    let web = short.open_webpage("https://g1.globo.com/bemestar/coronavirus/noticia/2020/04/27/brasil-tem-4543-mortes-e-66501-casos-de-coronavirus-diz-ministerio.ghtml", None);
    short.map_webpage(web.unwrap());

    // let file = short.open_file("/home/darksrc/xbox.sh");

    // short.read_file(file.unwrap());

    short.analyze(ReadType::Test)

}

