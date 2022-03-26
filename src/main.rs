extern crate rand;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use rand::Rng;

const MAX_ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    is_revealed: bool,
}

enum GameState {
    InProgress,
    Won,
    Lost
}

impl GameState {
    fn check_game_state(attempts_left: u8, letters: &[Letter]) -> GameState {
        if attempts_left <= 0 {
            return GameState::Lost;
        }

        if letters.iter().all(|letter| letter.is_revealed == true) {
            return GameState::Won;
        }

        return GameState::InProgress;
    }
}

fn main() {
    let word = select_word();
    let mut letters = word_to_letters(&word);

    let mut attempts_left: u8 = MAX_ALLOWED_ATTEMPTS;

    loop {
        let display_string = letters_to_display(&letters);

        println!("Display string is: {}", display_string);
        println!("You have {} attempts_left", attempts_left);
        println!("Enter your guess");

        let user_input = read_input_character();

        if user_input == '*' {
            println!("Exit code called!");
            break;
        }

        let mut user_guessed_correctly = false;

        for letter in letters.iter_mut() {
            if letter.character == user_input {
                letter.is_revealed = true;
                user_guessed_correctly = true;
            }
        }

        if user_guessed_correctly == false {
            println!("You guessed incorrectly! Your guess was {}.", user_input);

            attempts_left -= 1;
        }

        match GameState::check_game_state(attempts_left, &letters) {
            GameState::Won => {
                println!("Congratulations! You won! The word was {}.", word);
                break;
            }
            GameState::Lost => {
                println!("Sorry, but you're out of tries!");
                break;
            }
            GameState::InProgress => {
                continue;
            }
        }
    }
}

fn select_word() -> String {
    let mut file = File::open("words.txt").expect("Could not open file!");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect("Something went wrong when reading the file contents!");

    let available_words: Vec<&str> = file_contents.trim().lines().collect();
    let random_index = rand::thread_rng().gen_range(0..available_words.len());

    String::from(available_words[random_index])
}

fn word_to_letters(word: &str) -> Vec<Letter> {
    return word
        .chars()
        .map(|letter| Letter { character: letter, is_revealed: false })
        .collect();
}

fn letters_to_display(letters: &[Letter]) -> String {
    return letters
        .iter()
        .map(|letter| {
            if letter.is_revealed {
                return String::from(letter.character)
            }

            String::from('_')
        })
        .collect::<Vec<String>>().join(" ")
}

fn read_input_character() -> char {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.chars().next() {
                Some(c) => c,
                None => '*',
            }
        }
        Err(_) => '*'
    }

}
