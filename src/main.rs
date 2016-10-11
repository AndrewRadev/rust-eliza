extern crate linenoise;
extern crate regex;
extern crate rand;

use regex::Regex;
use rand::Rng;

// TODO: Filler words/phrases in input: "I guess", "I suppose", "maybe", "well,", "to be honest"

fn main() {
    println!("Hello, I'm Eliza, your psychiatrist. How are you feeling today?");

    loop {
        match linenoise::input("> ") {
            None => break,
            Some(input) => {
                if input == "quit" {
                    break;
                } else {
                    linenoise::history_add(&input);
                    println!("{}", build_response(input));
                }
            },
        }
    }
}

fn build_response(input: String) -> String {
    let mut rng = rand::thread_rng();

    let prefix_choices =
        [
        "Interesting, you say that",
        "Are you sure",
        "Are you sure that",
        "How do you feel about the fact that",
        "Does it bother you that",
        ];
    let prefix = rng.choose(&prefix_choices).unwrap();

    let suffix_choices =
        [
        "Tell me more.",
        "Elaborate, please.",
        "",
        "",
        ];
    let suffix = rng.choose(&suffix_choices).unwrap();

    return format!("{} {}? {}", prefix, reverse_input(input), suffix);
}

fn reverse_input(input: String) -> String {
    // Normalize input:
    let response = input.to_lowercase();
    let response = Regex::new(r"\s+").unwrap().replace_all(&response, " ");

    let mut rng = rand::thread_rng();

    let &replacement = rng.choose(&["you are", "you're"]).unwrap();
    let response = Regex::new(r"\b(i'm|i am)\b").unwrap().replace_all(&response, replacement);

    let &replacement = rng.choose(&["you would", "you'd"]).unwrap();
    let response = Regex::new(r"\b(i'd|i would)\b").unwrap().replace_all(&response, replacement);

    let response = Regex::new(r"\bi\b").unwrap().replace_all(&response, "you");
    let response = Regex::new(r"\bme\b").unwrap().replace_all(&response, "you");

    return response;
}
