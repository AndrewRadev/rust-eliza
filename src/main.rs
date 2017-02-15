extern crate linenoise;
extern crate eliza;

// TODO: Instead of a single function, consider an object with state that remembers the
// conversation.

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
                    println!("{}", eliza::get_response_to(input));
                }
            },
        }
    }
}
