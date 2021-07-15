#[macro_use]
extern crate clap;
use clap::App;

mod emojify;

fn main() {
    let yaml = load_yaml!("config/cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    let sentence: String = String::from(args.value_of("sentence").unwrap());
    let colours: Vec<String> = args.values_of("modifiers").unwrap_or_default().map(|x| String::from(x)).collect();
    let emojified_sentence = emojify::sentence_to_emoji(sentence, colours);
    print!("{}", emojified_sentence.sentence)
}

