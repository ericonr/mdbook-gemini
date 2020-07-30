use env_logger;
use log::{debug, info, trace};
use mdbook::book::BookItem;
use mdbook::book::Chapter;
use mdbook::errors::Result;
use mdbook::renderer::RenderContext;
use std::io;

mod fs;
mod gemini;

fn main() {
    env_logger::init();
    generate();
}

fn generate() -> Result<()> {
    info!("Starting the gemini generator");
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    // counting impl
    fn count_words(ch: &Chapter) -> usize {
        ch.content.split_whitespace().count()
    }

    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            let num_words = count_words(ch);
            println!("{}: {}", ch.name, num_words);
        }
    }

    let destination = &ctx.destination;
    let book = &ctx.book;

    for item in book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if !ch.is_draft_chapter() {
                let mut filename = ch.path.as_ref().unwrap().clone();
                filename.set_extension("gmi");
                let rendered = gemini::render(ch.content.clone());
                fs::write_file(destination, &filename, rendered.as_bytes())?;
            }
        }
    }

    Ok(())
}
