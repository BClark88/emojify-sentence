use rand::seq::SliceRandom;

pub struct EmojifiedSentence {
    pub sentence: String
}

pub fn sentence_to_emoji(sentence: String) -> EmojifiedSentence {
    let mut emojified_sentence: String = "".to_string();
    sentence.chars().for_each(|character| match character {
        character if character.is_ascii() => emojified_sentence.push_str(&emojified_alpha(character)),
        character if character.is_whitespace() => emojified_sentence.push_str("        "),
        _ => (),
    });
    return EmojifiedSentence { sentence: emojified_sentence };
}

fn emojified_alpha(character: char) -> String {
    let colours = vec!["yellow", "white"];
    let random_colour = colours.choose(&mut rand::thread_rng()).unwrap();
    return format!(":alphabet-{}-{}", random_colour, character);
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_sentence_to_emoji() {
        let re = Regex::new(
                r"^:alphabet-(yellow|white)-a$"
            ).unwrap();
        let input = "a".to_owned();
        let output = sentence_to_emoji(input).sentence;
        assert!(re.is_match(&output));
    }

    #[test]
    fn test_ignores_non_ascii() {
        let re = Regex::new(
                r"^:alphabet-(yellow|white)-n$"
            ).unwrap();
        let input = "nñ".to_owned();
        let output = sentence_to_emoji(input).sentence;
        assert!(re.is_match(&output));
    }
}
