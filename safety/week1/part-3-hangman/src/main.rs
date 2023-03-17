// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    print!("\nWelcome to Hangman in terminal!\n");

    let mut remain_count: u32 = NUM_INCORRECT_GUESSES;
    let mut filled_count: usize = 0;
    let mut guess_history: Vec<char> = Vec::new();
    let mut cur_words_chars: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut game_is_over = false;

    while !game_is_over {
        let cur_word: String = cur_words_chars.iter().collect();
        print!("The word so far is {}\n", cur_word);

        let history_str: String = guess_history.iter().collect();
        print!("You have guessed the following letters: {}\n", history_str);

        print!("You have {} guesses left\n", remain_count);

        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error readling line.");

        let guess_chars: Vec<char> = guess.chars().collect();
        let guess = guess_chars[0]; // only accept the first char

        if secret_word_chars.contains(&guess) {
            let pos = secret_word
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == guess)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            for elem in pos.iter() {
                cur_words_chars[*elem] = guess;
                filled_count += 1;
            }

            if filled_count == secret_word_chars.len() {
                game_is_over = true;
            }
        } else {
            remain_count -= 1;

            if remain_count == 0 {
                game_is_over = true;
                print!("Sorry, that letter is not in the word\n");
            }
        }

        guess_history.push(guess);
        print!("\n")
    }

    if filled_count == secret_word_chars.len() {
        print!("Congratulations you guessed the secret word: lobster!\n");
    } else {
        print!("Sorry, you ran out of guesses!");
    }
}
