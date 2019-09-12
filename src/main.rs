// use serde::{Deserialize, Serialize};
// use serde_json::{Result, Error};

// #[derive(Serialize, Deserialize)]
// struct Tyler {
//     first_name: String,
//     last_name: String
// }

// const JSON: &str = r#"
//   {
//     "first_name": "Tyler",
//     "last_name": "Gross"
//   }
// "#;

// fn deserialze(json: &str) -> Result<Tyler, Error> {
//     let deserialzed_json = serde_json::from_str(json);
//     deserialzed_json
// }

// fn main() {
//     deserialze(JSON);
// }

// #[derive(PartialEq)]
// enum SpiderMen {
//     Peter,
//     Miles
// }

// struct Thing {
//     pep: String,
//     people: Vec<String>,
// }

// fn main() {
//     let x = Thing {
//         pep: String::from("Tyler"),
//         people: vec![String::from("Dad"), String::from("Mom")],
//     };
//     println!("\n I am {} \n {} \n {} \n", x.pep, x.people[0], x.people[1]);
//     println!("\n First spider is {}", SpiderMen::Peter);
// }

use colored::*;
use std::io::stdin;

// Convert some of *this* text to a bunch of |dong| emojies

enum State {
    Normal,
    Fire,
    Eggplant,
}

fn parse_character(state: State, character: char) -> (State, Option<char>) {
    use self::State::*;
    match (state, character) {
        (Normal, '|') => (Eggplant, None),
        (Normal, '*') => (Fire, None),
        (Normal, other) => (Normal, Some(other)),
        (Fire, '*') => (Normal, None),
        (Fire, _) => (Fire, Some('🔥')),
        (Eggplant, '|') => (Normal, None),
        (Eggplant, _) => (Eggplant, Some('🍆')),
    }
}

fn main() {
    println!("\n\n{}", "Enter some text to markup:".blue().bold());

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Somehow, there wasn't anything entered");
    let mut state = State::Normal;
    let mut output = String::new();
    for character in input.chars() {
        let (new_state, character_output) = parse_character(state, character);
        if let Some(c) = character_output {
            output.push(c);
        }

        state = new_state;
    }
    println!("{}", output);
}
