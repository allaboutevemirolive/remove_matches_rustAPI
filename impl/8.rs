pub struct RemovePatternIter<'a> {
    text: &'a str,
    pattern: &'a str,
    finished: bool,
}

impl<'a> Iterator for RemovePatternIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        // Handle the case when the pattern is an empty string
        if self.pattern.is_empty() {
            self.finished = true;
            return Some(self.text);
        }

        if let Some(pos) = self.text.find(self.pattern) {
            let non_match = &self.text[..pos];
            // Skip pattern and return remainder
            self.text = &self.text[pos + self.pattern.len()..];
            Some(non_match)
        } else {
            self.finished = true;
            Some(self.text)
        }
    }
}

pub trait RemovePattern<'a>: Sized {
    fn remove_pattern(self, pattern: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a>;
}

impl<'a> RemovePattern<'a> for &'a str {
    fn remove_pattern(self, pattern: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(RemovePatternIter {
            text: self,
            pattern,
            finished: false,
        })
    }
}

// Example usage
fn main() {
    let text_str = "Hello, **Rust**! How are you doing **today**?";
    let text_string = String::from("Hello, **Rust**! How are you doing **today**?");
    let pattern = "**";

    // Using &str as input data type
    let result_str: String = text_str
        .remove_pattern(pattern)
        .collect::<Vec<_>>()
        .concat()
        .to_string();
    println!("{}", result_str); // Output: "Hello, Rust! How are you doing today?"

    // Using String as input data type
    let result_string: String = text_string
        .as_str() // Convert String to &str
        .remove_pattern(pattern)
        .collect::<Vec<_>>()
        .concat()
        .to_string();
    println!("{}", result_string); // Output: "Hello, Rust! How are you doing today?"
}







// Test cases
#[test]
fn test_no_pattern_occurrence() {
    let text = "No pattern here.";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "No pattern here.");
}

#[test]
fn test_single_pattern_occurrence() {
    let text = "Pattern ** found.";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern  found.");
}

#[test]
fn test_multiple_pattern_occurrences() {
    let text = "Pattern ** found ** twice.";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern  found  twice.");
}

#[test]
fn test_pattern_at_beginning() {
    let text = "**Pattern at the beginning.";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern at the beginning.");
}

#[test]
fn test_pattern_at_end() {
    let text = "Pattern at the end.**";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern at the end.");
}

#[test]
fn test_empty_text() {
    let text = "";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "");
}

// TODO: stuck
#[test]
fn test_empty_pattern() {
    let text = "Testing with empty pattern.";
    let pattern = "";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, text);
}

#[test]
fn test_large_text_and_pattern() {
    let text = "Large text with multiple pattern occurrences: **Pattern** **Pattern** **Pattern**.";
    let pattern = "**Pattern**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Large text with multiple pattern occurrences:   .");
}

#[test]
fn test_pattern_not_found() {
    let text = "No pattern in this text.";
    let pattern = "missing";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, text);
}

#[test]
fn test_pattern_empty_text() {
    let text = "";
    let pattern = "something";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "");
}

#[test]
fn test_pattern_empty_text_and_pattern() {
    let text = "";
    let pattern = "";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "");
}

#[test]
fn test_single_character_pattern() {
    let text = "a b c d e";
    let pattern = " ";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "abcde");
}

#[test]
fn test_large_pattern() {
    let text = "aaaXbbbXcccXdddXeee";
    let pattern = "X";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "aaabbbcccdddeee");
}

#[test]
fn test_pattern_occurs_after_empty_string() {
    let text = "abc X defXghi";
    let pattern = "X";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "abc  defghi");
}

#[test]
fn test_multiple_consecutive_patterns() {
    let text = "Hello **world****today!**";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Hello worldtoday!");
}

#[test]
fn test_pattern_with_special_characters() {
    let text = "Testing with $$$ pattern $$$";
    let pattern = "$$$";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Testing with  pattern ");
}

#[test]
fn test_pattern_at_multiple_positions() {
    let text = "Pattern ** found ** multiple ** times ** in ** text.";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern  found  multiple  times  in  text.");
}

#[test]
fn test_pattern_with_digits() {
    let text = "123 ** 456 ** 789";
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "123  456  789");
}

// TODO: enable case-sensitive?
#[test]
fn test_case_insensitive_pattern() {
    let text = "CASE ** SeNsItIvE ** PaTtErN.";
    let pattern = "sEnSiTiVe";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "CASE ** SeNsItIvE ** PaTtErN.");
}