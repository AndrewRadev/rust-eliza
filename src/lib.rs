extern crate regex;
extern crate rand;

use regex::Regex;
use rand::Rng;

pub fn get_response_to(input: String) -> String {
    let mut rng = rand::thread_rng();
    let normalized_input = normalize_input(&input);

    debug("normalized input", &normalized_input);

    if normalized_input.ends_with("?") {
        // TODO multiple randomized responses to this
        return "I'd like to be the one to ask the questions. Tell me more about yourself, instead.".to_string();
    }

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

    let reversed_input = reverse_pronouns(normalized_input);
    debug("reversed input", &reversed_input);

    // post-processing
    let reversed_input = replace_phrases(&reversed_input, r"it's true that|do|well|you guess|you suppose|kinda|sorta|thank you|please|yes|no", &[""]);

    // clean punctuation
    let reversed_input = Regex::new(r"\s+([,;-])").unwrap().replace_all(&reversed_input, "$1");
    let reversed_input = Regex::new(r"([,;-])+"  ).unwrap().replace_all(&reversed_input, "$1");
    let reversed_input = Regex::new(r"[,;-]+\s*$").unwrap().replace_all(&reversed_input, "");
    let reversed_input = Regex::new(r"\s\s+$"    ).unwrap().replace_all(&reversed_input, " ");
    let reversed_input = reversed_input.trim().to_string();

    debug("cleaned reversed input", &reversed_input);

    return format!("{} {}? {}", prefix, reversed_input, suffix);
}

// Utility functions

fn reverse_pronouns(input: String) -> String {
    let response = input;
    let response = replace_phrases(&response, r"i'm|i am",    &["you are",   "you're"]);
    let response = replace_phrases(&response, r"i'd|i would", &["you would", "you'd"]);
    let response = replace_phrases(&response, r"i|me",        &["you"]);
    let response = replace_phrases(&response, r"my",          &["your"]);
    let response = replace_phrases(&response, r"mine",        &["yours"]);

    return response;
}

fn replace_phrases(input: &str, pattern: &str, replacement: &[&str]) -> String {
    let mut rng      = rand::thread_rng();
    let &replacement = rng.choose(replacement).unwrap();
    let full_pattern = format!(r"\b({})\b", pattern);
    let response     = Regex::new(&full_pattern).unwrap().replace_all(&input, replacement);

    return response;
}

fn normalize_input(input: &str) -> String {
    let normalized_input = input;
    let normalized_input = normalized_input.to_lowercase().trim().to_string();

    let normalized_input = Regex::new(r"\s+"   ).unwrap().replace_all(&normalized_input, " ");
    let normalized_input = Regex::new(r"[.?!]" ).unwrap().replace_all(&normalized_input, "");
    let normalized_input = Regex::new(r"\s\s+$").unwrap().replace_all(&normalized_input, " ");

    return normalized_input;
}

// Debugging messages only in Debug target, not Release

#[cfg(debug_assertions)]
fn debug(label: &str, subject: &str) {
    println!("DEBUG ({}), {:?}", label, subject);
}

#[cfg(not(debug_assertions))]
fn debug(label: &str, subject: &str) {
    // do nothing;
}
