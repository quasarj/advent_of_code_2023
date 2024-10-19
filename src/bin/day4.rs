use log;
use logos::Logos;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("Card")]
    Card,

    #[token("|")]
    Pipe,

    // Or regular expressions.
    #[regex("\\d+")]
    Number,

    #[regex("\\d+:")]
    CardNumber,
}

fn calc_score(card_line: &str) -> u64 {
    let mut lex = Token::lexer(card_line);

    // TODO move this to a function to process each row of input
    let mut winning_numbers: HashSet<u64> = HashSet::new();
    let mut my_numbers: HashSet<u64> = HashSet::new();

    let mut pipe_seen = false;

    loop {
        let token = lex.next();
        let value = lex.slice();

        match token {
            None => break,
            // Some(Ok(Token::Card)) => println!("Card: {}", value),
            // Some(Ok(Token::CardNumber)) => println!("Card Number: {}", value),
            Some(Ok(Token::Pipe)) => pipe_seen = true,
            Some(Ok(Token::Number)) => {
                if pipe_seen {
                    my_numbers.insert(value.parse().expect("should have been a number"));
                } else {
                    winning_numbers.insert(value.parse().expect("should have been a number"));
                }
            }
            _ => {}
        }
    }
    log::debug!("{:?}", winning_numbers);
    log::debug!("{:?}", my_numbers);

    let my_winning_numbers: Vec<_> = winning_numbers.intersection(&my_numbers).copied().collect();
    println!("{:?}", my_winning_numbers);

    if my_winning_numbers.is_empty() {
        0
    } else {
        u64::pow(2, (my_winning_numbers.len() - 1) as u32)
    }
}

fn main() {
    env_logger::init();

    // let filename = "input/day4-test.txt";
    let filename = "input/day4.txt";

    log::info!("Using filename={filename}");

    let binding = read_to_string(filename).unwrap();
    let score: u64 = binding.lines().map(|x| calc_score(x)).sum();
    println!("{}", score);
}
