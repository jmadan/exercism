pub fn reply(message: &str) -> &str {
    println!("----{}----", message);
    match message.trim() {
        m  if m.chars().any(|c| c.is_ascii_alphabetic())
            && m.to_uppercase().eq(message)
            && m.trim().ends_with("?")  => "Calm down, I know what I'm doing!",
        m if m.trim().ends_with("?") => "Sure.",
        m if !message.chars().any(|c| c.is_ascii_alphanumeric()) => "Fine. Be that way!",
        m if message.chars().any(|c| c.is_ascii_alphabetic()) && message.to_uppercase().eq(message) => "Whoa, chill out!",
        _ => "Whatever."
    }
}
