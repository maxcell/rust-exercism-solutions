use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut bucket: HashSet<char> = HashSet::new();
    for curr_char in candidate.to_ascii_lowercase().chars() {
        if curr_char == ' ' || curr_char == '-' {
            continue;
        }
        if bucket.contains(&curr_char) {
            return false;
        }
        bucket.insert(curr_char);
    }
    true
}
