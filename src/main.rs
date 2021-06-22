struct Sentence {
    sentence: String
}
fn main() {
    let arg = std::env::args().nth(1).expect("No sentence supplied");
    let pattern = String::from(arg);
    let sentence = Sentence { sentence: pattern };
    println!("{}", sentence.sentence);
}
