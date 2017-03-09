extern crate regex;
extern crate rand;

use regex::Regex;
use rand::Rng;

pub fn get_response_to(input: String) -> String {
    let mut rng = rand::thread_rng();
    let normalized_input = normalize_input(&input);

    debug("normalized input", &normalized_input);

    if normalized_input.ends_with("?") {
        // It's a question, let's see if we can turn it around
        let responses = [
            "I'd like to be the one to ask the questions. Tell me more about yourself, instead.",
            "What makes you ask that?",
            "I'm not sure I understand the question. Could you elaborate?",
            "What do you mean?"
        ];
        return rng.choose(&responses).unwrap().to_string();
    }

    if rng.gen_weighted_bool(3) {
        // Every 1 in 3 times, just add some filler
        let responses = [
            "Tell me more.",
            "Elaborate, please.",
            "Do continue.",
            "Go on.",
            "I see. How does that make you feel?",
            "I understand. What do you think about that?",
            "How interesting. What else can you tell me about that?"
        ];
        return rng.choose(&responses).unwrap().to_string();
    }

    // Reverse user input with a prefix
    let prefix_choices = [
        "Interesting, you say that",
        "Are you sure",
        "Are you sure that",
        "How do you feel about the fact that",
        "Does it bother you that",
    ];
    let prefix = rng.choose(&prefix_choices).unwrap();

    let reversed_input = reverse_pronouns(normalized_input);
    debug("reversed input", &reversed_input);

    // post-processing
    let filler_phrases = [
        r"it's true that",
        r"you guess", r"you suppose",
        r"kinda", r"sorta",
        r"thank you", r"please", r"yes", r"no"
    ];
    let reversed_input = replace_phrases(&reversed_input, &filler_phrases, &[""]);

    // clean punctuation
    let reversed_input = Regex::new(r"\s+([,;-])").unwrap().replace_all(&reversed_input, "$1");
    let reversed_input = Regex::new(r"([,;-])+"  ).unwrap().replace_all(&reversed_input, "$1");
    let reversed_input = Regex::new(r"^\s*[,;-]" ).unwrap().replace_all(&reversed_input, "");
    let reversed_input = Regex::new(r"[,;-]+\s*$").unwrap().replace_all(&reversed_input, "");
    let reversed_input = Regex::new(r"\s\s+$"    ).unwrap().replace_all(&reversed_input, " ");
    let reversed_input = reversed_input.trim().to_string();

    debug("cleaned reversed input", &reversed_input);

    return format!("{} {}?", prefix, reversed_input);
}

// Utility functions

fn reverse_pronouns(input: String) -> String {
    let response = input;
    let response = replace_phrases(&response, &[r"i'm", r"i am"],    &["you are", "you're"]);
    let response = replace_phrases(&response, &[r"i'd", r"i would"], &["you would", "you'd"]);
    let response = replace_phrases(&response, &[r"i", r"me"],        &["you"]);
    let response = replace_phrases(&response, &[r"my"],              &["your"]);
    let response = replace_phrases(&response, &[r"mine"],            &["yours"]);

    return response.to_string();
}

fn replace_phrases(input: &str, patterns: &[&str], replacement: &[&str]) -> String {
    let mut rng      = rand::thread_rng();
    let &replacement = rng.choose(replacement).unwrap();
    let full_pattern = format!(r"\b({})\b", patterns.join("|"));
    let response     = Regex::new(&full_pattern).unwrap().replace_all(&input, replacement);

    return response.to_string();
}

fn normalize_input(input: &str) -> String {
    let mut rng = rand::thread_rng();

    // First, split into sentences, to pick just one to respond to
    let sentence_regex = Regex::new(r".*?([.?!]|$)").unwrap();
    let sentences: Vec<&str> = sentence_regex.
        find_iter(input).
        map( |m| m.as_str() ).
        collect();
    debug("sentences", &sentences);
    let target_sentence = rng.choose(&sentences).unwrap();

    let normalized_input = target_sentence;
    let normalized_input = normalized_input.to_lowercase().trim().to_string();

    let normalized_input = Regex::new(r"\s+"   ).unwrap().replace_all(&normalized_input, " ");
    let normalized_input = Regex::new(r"[.!]"  ).unwrap().replace_all(&normalized_input, "");
    let normalized_input = Regex::new(r"\s\s+$").unwrap().replace_all(&normalized_input, " ");

    return normalized_input.to_string();
}

// Debugging messages only in Debug target, not Release

#[cfg(debug_assertions)]
fn debug<T>(label: &str, subject: T)
    where T: std::fmt::Debug {
    println!("DEBUG ({}), {:?}", label, subject);
}

#[cfg(not(debug_assertions))]
fn debug<T>(label: &str, subject: T)
    where T: std::fmt::Debug {
    // do nothing;
}
