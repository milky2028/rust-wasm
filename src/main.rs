use colored::*;
use rand::random;
use std::io;
// use serde_json::{Result, Value};
// use wasm_bindgen::prelude::*;

// To create functions suitable for web assembly:
// Add these to cargo.toml
// [lib]
// crate-type = ["cdylib", "rlib"]

// some demos for deserializing data to a strictly-typed structure
// struct JsonData {
//   userId: u8,
//   id: u8,
//   title: String,
//   completed: bool
// }

// Add this above and function you want to make available to wasm_bindgen
// #[wasm_bindgen]
// pub fn weighted_average(json: &str) -> JsonData {
//   let val: JsonData = serde_json::from_str(json);
//   val
// }

// fn main() {
//   println!(
//     "{}",
//     weighted_average("{\"userId\":1,\"id\":1,\"title\":\"delectus aut autem\",\"completed\":false}")
//   );
// }

fn get_user_guess() -> u8 {
    loop {
        println!("{}\n", "Make a guess:".bold().blue());
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Could not read input");
        user_guess
            .trim()
            .parse::<u8>()
            .expect("Did you enter a number?");
    }

    // let mut guess_display = String::new();
    // if user_guess_as_number == correct_answer {
    //     guess_display = user_guess.green().to_string();
    // } else {
    //     guess_display = user_guess.red().to_string();
    // }

    // println!("{} {}", "You guessed:", guess_display)
}

fn main() {
    println!("\n\n\n{}", "This is a guessing game!".bold().purple());
    let correct_answer = random::<u8>();
    println!(
        "\n\n{} {}",
        "The correct answer is:".bold(),
        correct_answer.to_string().green().bold()
    );
    let a = get_user_guess();
}
