pub fn reply(message: &str) -> &str {
    println!("----{}----", message);
    if message.chars().any(|c| c.is_ascii_alphabetic())
        && message.to_uppercase().eq(message)
        && message.trim().ends_with("?")
    {
        return "Calm down, I know what I'm doing!";
    }
    if message.trim().ends_with("?") {
        return "Sure.";
    }

    if !message.chars().any(|c| c.is_ascii_alphanumeric()) {
        return "Fine. Be that way!"
    }

    if message.chars().any(|c| c.is_ascii_alphabetic()) && message.to_uppercase().eq(message) {
        return "Whoa, chill out!"
    }

    "Whatever."
}
