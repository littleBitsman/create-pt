use std::{fmt::Display, io::{stdin, stdout, Result as IoResult, Write}};
use colored::Colorize;
use rand::{thread_rng, Rng};

const WORDS_FILE: &'static str = include_str!("words.txt");
const ANSWER_FILE: &'static str = include_str!("answers.txt");

const GUESSES: usize = 6;

fn print(s: impl Display) {
    print!("{}", s);
    stdout().flush().unwrap();
}

fn read_line_stdin() -> IoResult<String> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}

#[allow(dead_code)]
struct Game {
    correct_word: String,
    guesses: Vec<String>,
    possible_words: Vec<String>,
    has_won: bool
}

#[allow(dead_code)]
impl Game {
    pub fn new() -> Self {
        let mut words = WORDS_FILE.lines().map(|v| v.to_lowercase()).collect::<Vec<String>>();
        let answers = ANSWER_FILE.lines().map(|v| v.to_lowercase()).collect::<Vec<String>>();
        words.append(&mut answers.clone());
        Self {
            correct_word: answers.get(thread_rng().gen_range(0..answers.len())).expect("well thats awkward").clone(),
            guesses: vec![],
            possible_words: words,
            has_won: false
        }
    }

    pub fn get_correct_word(&self) -> String {
        self.correct_word.clone()
    }

    pub fn get_guess_count(&self) -> usize {
        self.guesses.len()
    }

    pub fn submit_guess(&mut self, guess: String) -> bool {
        if guess.len() != 5 {
            println!("{}", "your guess must be 5 letters long".red().italic())
        } else if self.possible_words.iter().find(|v| **v == guess.to_lowercase()).is_none() {
            println!("{}", "invalid word, try again".red().italic())
        } else {
            self.guesses.push(guess.to_lowercase());
            if guess.to_lowercase() == self.correct_word.to_lowercase() { // yay winner!
                println!("{}", guess.to_lowercase().green().bold());
                self.has_won = true;
                return true
            }
            let guess_chars = guess.to_lowercase().chars().collect::<Vec<char>>();
            let correct_chars = self.correct_word.chars().collect::<Vec<char>>();

            let mut final_str = String::new();

            for i in 0..=4 {
                let guess_char = guess_chars.get(i).expect("how");
                let correct_char = correct_chars.get(i).expect("how");

                if guess_char == correct_char {
                    final_str = format!("{}{}", final_str, guess_char.to_string().green().bold());
                } else if correct_chars.iter().find(|v| v == &guess_char).is_some() && correct_chars.iter().fold(0, |acc, v| if v == guess_char { acc + 1 } else { acc }) > i {
                    final_str = format!("{}{}", final_str, guess_char.to_string().yellow().bold());
                } else {
                    final_str = format!("{}{}", final_str, guess_char.to_string().black());
                }
            }

            println!("{}", final_str);
        }
        false
    }
}

fn main() {
    let mut wordle = Game::new();

    println!("TEST: {}", wordle.get_correct_word());

    println!("{}", "welcome to wordle but its in rust".bold());
    while wordle.get_guess_count() < GUESSES {
        print("type your guess: ".italic());
        if let Ok(input) = read_line_stdin() {
            wordle.submit_guess(input);
        } else {
            println!("{}", "type something to guess before pressing enter".red().italic())
        }
    }
}
