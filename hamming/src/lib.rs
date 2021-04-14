/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { None } else {
        let pairs = s1.chars().zip(s2.chars());
        let sum = pairs.fold(0, |mut acc, (char1, char2)| {
            if char1 != char2 { acc += 1; }
            acc
        });
        Some(sum)
    }
}

// zip, chars
// fold
// 