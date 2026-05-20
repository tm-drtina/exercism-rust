enum State {
    NewWord,
    InWord(char),
}

pub fn abbreviate(phrase: &str) -> String {
    phrase.chars().fold((String::new(), State::NewWord), |(mut res, state), ch| {
        match state {
            State::NewWord if ch.is_alphabetic() => {
                res.push(ch.to_ascii_uppercase());
                (res, State::InWord(ch))
            },
            State::NewWord => (res, State::NewWord),
            _ if ch == ' ' || ch == '-' => (res, State::NewWord),
            State::InWord(prev) if prev.is_ascii_lowercase() && ch.is_ascii_uppercase() => {
                res.push(ch);
                (res, State::InWord(ch))
            },
            State::InWord(_) => (res, State::InWord(ch)),
        }
    }).0
}
