extern crate google_translate3;

use google_translate3::Translator;

fn main() {
    let translator = Translator::new("en","zn");
    let translation = translator.translate("Hello");
    println!("{}", translation); // Bonjour
}
