use std::usize;

const ANSWER_FOR_ALL: u8 = 42;
const QUESTION_OF_ALL: &str = "life, the universe and everything";
const OK: &str = "Ok";

fn main() {
    let mut input = String::new();

    let mut questionarie = vec![
        (QUESTION_OF_ALL.to_string(), ANSWER_FOR_ALL.to_string()),
    ];

    let mut counter = 0;

    let mut r#continue = true;

    'count: while r#continue {
        input.clear();
        r#continue = iterate(&mut input, &mut questionarie);

        counter += 1;

        if counter > 3 {
            println!("You asking much");
            break 'count;
        }
    };

    println!("You have asked {counter} questions");
}

fn iterate(choice_buffer: &mut String, questionarie: &mut Vec<(String, String)>) -> bool {
    println!("What question do you want to ask?");
    println!("0. Stop asking questions");

    let own_choice = questionarie.len() + 1; 

    for (i, (question, answer)) in questionarie.iter().enumerate() {
        println!("{}: {} -> {}", i + 1, question, answer);
    }

    println!("{}. Your question", own_choice);

    match parse_natural_number(&read_line(choice_buffer)) {
        Ok(choice) => {
            if choice == 0 {
                println!("Bue");
                return false;
            }

            if choice > own_choice {
                eprintln!("Unexpected choice");
                return true;
            } 
            
            let (question, answer) = if choice == own_choice { add_question(questionarie) } else { get_question(questionarie, choice) };

            write_answer(answer, question);
        }
        Err(str) => {
            eprintln!("Unexpected input, {str}");

        }
    }

    true
}

fn add_question(questionarie: &mut Vec<(String, String)>) -> (&String, &String) {
    println!("What is your question?");
    let user_question = read_line(&mut String::new());
    println!("What is your answer?");
    let user_answer = read_line(&mut String::new());
    // questionarie.push((user_question.clone(), user_answer.clone()));
    questionarie.push((user_question, user_answer));

    let (q, a) = questionarie.last().unwrap();

    (q, a)
}

fn get_question(questionarie: &mut Vec<(String, String)>, choice: usize) -> (&String, &String) {
    let (q, a) = questionarie.get(choice - 1).unwrap();

    (q, a)
}

fn read_line(buf: &mut String) -> String {
    buf.clear();

    std::io::stdin()
        .read_line(buf)
        .expect("Failed to read line");

    buf.trim().to_string()
}

fn parse_natural_number(input: &str) -> Result<usize, &str> {
    match input.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("Expected number")
    }
}

fn write_answer(answer: &str, question: &str) -> &'static str {
    println!("The answer to {} is: {}", question, answer);

    OK
}