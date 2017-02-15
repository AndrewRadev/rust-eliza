This project is a simple chat bot, the good old [ELIZA](https://en.wikipedia.org/wiki/ELIZA)
psychoterapist bot. It's a very, very simplified version of it that I'm writing as a way to learn
Rust. It's not intended for any serious use.

The main idea is that Eliza takes what you tell her and turns it around as a question, with a
prompt. A sample session might look like this:

```
Hello, I'm Eliza, your psychiatrist. How are you feeling today?
> I'm feeling fine.
Does it bother you that you are feeling fine? Tell me more.
> Well, maybe I feel kinda guilty for feeling fine.
How do you feel about the fact that , maybe you feel  guilty for feeling fine? Tell me more.
> Have I really deserved to feel fine?
How do you feel about the fact that have you really deserved to feel fine?
>
```

As you can see, there's problems with formatting, and sometimes the responses are clumsy. It's a
work in progress, but it's also not intended to ever be a particularly impressive project.

## Extension ideas

- Would be nice to write some tests, need a way to stub the RNG.
- Responses to particular topics, say, mentioning "Mother" would lead to "Tell me about your
  mother".
- Memory of previous lines, somehow. If the user's mother has already been discussed, maybe this
  could be referenced.
- More padding words. Maybe a file with a list? Use [build.rs](http://doc.crates.io/build-script.html#case-study-code-generation) to include JSON with the contents
- More formats of responses -- repeating the user's input over and over gets tedious, maybe
  sometimes, it makes sense to just say "Go on", or "Tell me more", especially for longer user
  inputs.
