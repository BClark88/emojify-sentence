mod emojify;

fn main() {
    let input_sentence = std::env::args().nth(1).expect("No sentence supplied");

    let colours = vec!["yellow", "white"];
    let emojified_sentence = emojify::sentence_to_emoji(input_sentence, colours);
    print!("{}", emojified_sentence.sentence)
}
