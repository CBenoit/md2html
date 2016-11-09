extern crate hoedown;

use hoedown::Html;
use hoedown::renderer::html::Flags;
use hoedown::Markdown;
use hoedown::Render;

use std::io;
use std::io::Read;

fn main() {
    let mut input = Vec::new();
    match io::stdin().read_to_end(&mut input) {
        Ok(_) => (),
        Err(error) => println!("Error: {}", error),
    }

    let markdown = Markdown::new(String::from_utf8(input).unwrap().as_str());

    let mut html = Html::new(Flags::empty(), 0);

    println!("{}", html.render(&markdown).to_str().unwrap());
}

