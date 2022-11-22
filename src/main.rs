use std::io;

fn get_user_input() -> String {

    let mut user_answer = String::new();

    // Might want to put this in a loop
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to readline");

    match user_answer.trim().to_string() {
        value => value,
        error => "FOOBAR".to_string(),
    };
    
    return user_answer;

}

fn main() {
    println!("Type something in");

    let neo_answer = get_user_input();

    println!("Your answer was: {neo_answer}");
}

// To Do:

// Create a function that interperates user input

// Create a function that serves the 'room' that the user is in

// create a loop for the program to run in

// create a function that handles changing rooms

// create a method that keeps track on the player's data

// save room data in JSON

// make JSON readable for the program

