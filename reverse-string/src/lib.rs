use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
    // let grapheme_input = input.graphemes(true).collect::<Vec<&str>>();
    // let mut reversed_string = String::new();

    // for c in grapheme_input.iter().rev() {
    //     reversed_string.push_str(c);
    // }

    // reversed_string
}
