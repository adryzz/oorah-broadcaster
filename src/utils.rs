pub fn is_valid_topic_id(input: &str) -> bool {
    for ch in input.chars() {
        if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '-' {
            return false;
        }
    }
    true
}
