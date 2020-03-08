use unicode_segmentation::UnicodeSegmentation;

/// collect
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
