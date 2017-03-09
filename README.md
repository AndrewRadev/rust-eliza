This project is a simple chat bot, the good old [ELIZA](https://en.wikipedia.org/wiki/ELIZA)
psychoterapist bot. It's a very, very simplified version of it that I'm writing as a way to learn
Rust. It's not intended for any serious use.

The main idea is that Eliza takes what you tell her and turns it around as a question, with a
prompt. A sample session might look like this:

```
Hello, I'm Eliza, your psychiatrist. How are you feeling today?
> I'm feeling fine.
Interesting, you say that you are feeling fine?
> I do, I'm very optimistic today.
Are you sure that you do, you're very optimistic today?
> Well, now that you're asking, I'm starting to doubt it.
Elaborate, please.
> I think I could have been more productive at work.
Are you sure you think you could have been more productive at work?
> Maybe. It's difficult to tell.
Tell me more.
> It's like, how can you really tell?
I'd like to be the one to ask the questions. Tell me more about yourself, instead.
>
```

As you can see, the responses can be quite clumsy, but it's pretty fun when they work out. It's a
work in progress, but it's also not intended to ever be a particularly impressive project.

## Extension ideas

- Would be nice to write some tests, need a way to stub the RNG.
- Responses to particular topics, say, mentioning "Mother" would lead to "Tell me about your
  mother".
- Memory of previous lines, somehow. If the user's mother has already been discussed, maybe this
  could be referenced.
- More padding words. Maybe a file with a list? Use [build.rs](http://doc.crates.io/build-script.html#case-study-code-generation) to include JSON with the contents
