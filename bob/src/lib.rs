pub fn reply(message: &str) -> &str {
            let mut m = message.chars().filter(|x| !x.is_ascii_whitespace()).collect::<Vec<char>>();

    if m.iter().last().is_none() {"Fine. Be that way!"}
    else if m.pop() == Some('?') {
        if m.iter().all(|x|x.is_ascii_uppercase() | x.eq(&'\''))  {"Calm down, I know what I'm doing!"}
        else {"Sure."}
    }
        else if m.iter().all(|x|!x.is_ascii_lowercase()) & m.iter().any(|x|x.is_ascii_alphabetic()) {"Whoa, chill out!"}
    else {"Whatever."}
}
