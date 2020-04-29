use short_text::{ShortText, WebPageInterface, ShortTextFunctions, MetaExtractionStrategy};
use short_text::objects::content::ContentFunctions;

fn main() {
    let short = ShortText::new();
    // short.set_meta_extraction_strategy(MetaExtractionStrategy::Twitter);
    let web = short.open_webpage("https://ensinarhistoriajoelza.com.br/iluminismo-do-antigo-regime-aos-nossos-dias/", None);
    let content = short.map_webpage(web.unwrap());

    println!("title: {}\n\
              description: {}\n\
              image: {}",
             content.get_title(),
             content.get_description(),
             content.get_thumbnail()
    )
}

