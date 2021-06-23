mod emojify;

fn main() {
    let arg = std::env::args().nth(1).expect("No sentence supplied");
    let pattern = String::from(arg);
    let emojified_sentence = emojify::sentence_to_emoji(pattern);
    print!("{}", emojified_sentence.sentence)
}

