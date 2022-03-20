extern crate rand;

use std::fs::File;
use std::io::prelude::*;

use rand::Rng;

struct Letter {
    character: char,
    is_revealed: bool,
}

fn main() {
    let word = select_word();
    let letters = word_to_letters(&word);
    let display_string = letters_to_display(&letters);

    println!("Word is {}, display string is {}", word, display_string);
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
