pub fn find_line<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn find_line_ci<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::<&str>::new();
    let query = query.to_lowercase();

    content.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const POEM: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";


    // test find_line function case
    fn test_find_line_case<'a>(query: &str, content: &'a str, expected: Vec<&'a str>) {
        let result = find_line(query, content);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_line() {
        test_find_line_case("fish", POEM, Vec::new());
        test_find_line_case("frog", POEM, vec!["How public, like a frog"]);
        test_find_line_case("nobody", POEM, vec![
            "I'm nobody! Who are you?",
            "Are you nobody, too?",
        ]);
        test_find_line_case("dog", "", Vec::new());
        test_find_line_case("Frog", POEM, Vec::new());
        // test_find_line_case("", content, );

    }

    // test find_line_ci function case
    fn test_find_line_ci_case<'a>(query: &str, content: &'a str, expected: Vec<&'a str>) {
        let result = find_line_ci(query, content);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_line_ci() {
        test_find_line_ci_case("FRoG", POEM, vec!["How public, like a frog"]);
    }
}
