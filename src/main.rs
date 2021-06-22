use regex::Regex;

struct Sentence {
    sentence: String
}
fn main() {
    let arg = std::env::args().nth(1).expect("No sentence supplied");
    let pattern = String::from(arg);
    let sentence = Sentence { sentence: pattern };
    // println!("{}", sentence_to_emoji(sentence.sentence));
    sentence_to_emoji(sentence.sentence);
}

// fn sentence_to_emoji(sentence: String) -> String {
fn sentence_to_emoji(sentence: String) {
    let re = Regex::new(r"(?P<character>[[:alpha:]])").unwrap();
    for cap in re.captures_iter(&sentence) {
        let emojified = format!(":alphabet-yellow-{}:", &cap["character"]);
        print!("{}", emojified)
    }
}
