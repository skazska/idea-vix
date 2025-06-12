enum CharType {
    Vowel,
    Consonant(String),
    Empty
}

const VOWELS: &str = "aeiouAEIOU";
const OMIT_CONS: &str = "hH";

/**
 * Returns if character is a vowel.
 */
#[inline]
fn is_vowel(c: char) -> bool { VOWELS.contains(c) }

/**
 * Returns if character is a omit consonant.
 */
#[inline]
fn is_omit_cons(c: char) -> bool { OMIT_CONS.contains(c) }

/**
 * Returns a typed first letter.
 */
fn get_typed_char(word: &str) -> CharType {
    let mut chars = word.chars();

    let Some(c) = chars.next() else { return CharType::Empty };

    if !c.is_ascii_alphabetic() { return CharType::Empty }; 

    if is_vowel(c) || is_omit_cons(c) { return CharType::Vowel };

    let mut cons = String::from(c);

    if let Some(nc) = chars.next() {
        if nc.is_ascii_alphabetic() && !is_vowel(nc) { cons.push(nc) }; 
    } 

    CharType::Consonant(cons)
}

/**
 * Returns .
 */


/**
 * Convert strings to pig latin. 
 * The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
 * Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
 * Keep in mind the details about UTF-8 encoding!
 */
pub fn to_pig_latin(s: &str) -> String {
    let words = s.split_inclusive(|c: char| !c.is_alphanumeric())
        .map(|part| {
            let (prefix, word) = part.split_at(part.find(|c: char| c.is_alphabetic()).unwrap_or(part.len()));
            (prefix, get_typed_char(word), word)
        })
        .map(|(prefix, first, word)| {
            let mut chars = word.chars();

            let suffix = match first {
                CharType::Consonant(cons) => {
                    for _ in 0..cons.len() { chars.next(); } 

                    format!("{}ay", cons)
                },
                CharType::Vowel => "ay".to_string(),
                _ => String::new(),
            };

            let mut pig_latin_word = String::new();
            let mut ending = String::new();

            loop {
                match chars.next() {
                    Some(c) if c.is_ascii_alphabetic() => { pig_latin_word.push(c); }
                    Some(c) => { ending.push(c); }
                    None => break,
                }
            }

            if pig_latin_word.is_empty() {
                format!("{}{}{}", prefix, suffix, ending)
            } else {
                format!("{}{}-{}{}", prefix, pig_latin_word, suffix, ending)
            }

        })
        .filter(|word| !word.is_empty())
        .collect();

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_to_pig_latin_case(r#in: &str, expected: &str) {
        let result = to_pig_latin(r#in);
        assert_eq!(result, expected, "Inpit: '{}', Expected: '{}', Got: '{}'", r#in, expected, result);
    }

    #[test]
    fn test_to_pig_latin() {
        test_to_pig_latin_case("first", "irst-fay");
        test_to_pig_latin_case("apple", "apple-ay");
        test_to_pig_latin_case("honest", "honest-ay");
        test_to_pig_latin_case("cherry4", "erry-chay4");

        test_to_pig_latin_case("banana, the best!", "anana-bay, e-thay est-bay!");
        test_to_pig_latin_case("4chan", "4an-chay");
        test_to_pig_latin_case("12345", "12345");
        test_to_pig_latin_case("", "");
    }
}