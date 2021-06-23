use rand::seq::SliceRandom;

pub struct EmojifiedSentence {
    pub sentence: String,
}

pub fn sentence_to_emoji(sentence: String, colour_options: Vec<&str>) -> EmojifiedSentence {
    let mut emojified_sentence: String = "".to_string();
    sentence.chars().for_each(|character| match character {
        character if character.is_ascii_alphanumeric() => {
            emojified_sentence.push_str(&emojified_alpha(character, colour_options.clone()))
        }
        character if character.is_whitespace() => emojified_sentence.push_str("        "),
        _ => (),
    });
    return EmojifiedSentence {
        sentence: emojified_sentence,
    };
}

fn emojified_alpha(character: char, colour_options: Vec<&str>) -> String {
    let random_colour = colour_options.choose(&mut rand::thread_rng()).unwrap();
    return format!(":alphabet-{}-{}:", random_colour, character);
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_sentence_to_emoji() {
        let re = Regex::new(r"^:alphabet-(yellow|white)-a:$").unwrap();
        let colours = vec!["yellow", "white"];
        let input = "a".to_owned();
        let output = sentence_to_emoji(input, colours).sentence;
        assert!(re.is_match(&output));
    }

    #[test]
    fn test_expands_spaces() {
        // an a, eight spaces and another a
        let re =
            Regex::new(r"^:alphabet-(yellow|white)-a:\s{8}:alphabet-(yellow|white)-a:$").unwrap();
        let colours = vec!["yellow", "white"];
        let input = "a a".to_owned();
        let output = sentence_to_emoji(input, colours).sentence;
        assert!(re.is_match(&output), "{}", output);
    }

    #[test]
    fn test_ignores_non_ascii() {
        let re = Regex::new(r"^:alphabet-(yellow|white)-n:$").unwrap();
        let colours = vec!["yellow", "white"];
        let input = "nñ".to_owned();
        let output = sentence_to_emoji(input, colours).sentence;
        assert!(re.is_match(&output));
    }
}
