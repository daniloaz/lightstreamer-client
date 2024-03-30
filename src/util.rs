/// Clean the message from newlines and carriage returns and convert it to lowercase.
pub fn clean_message(text: &str) -> String {
    text.replace("\n", "").replace("\r", "").to_lowercase()
}