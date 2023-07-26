pub struct RemovePatternIter {
    text: String,
    pattern: String,
    finished: bool,
}

impl Iterator for RemovePatternIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        // Handle the case when the pattern is an empty string
        if self.pattern.is_empty() {
            self.finished = true;
            return Some(self.text.clone());
        }

        if let Some(pos) = self.text.find(&self.pattern) {
            let non_match = self.text[..pos].to_string();
            // Skip pattern and return remainder
            self.text = self.text[pos + self.pattern.len()..].to_string();
            Some(non_match)
        } else {
            self.finished = true;
            Some(self.text.clone())
        }
    }
}

pub trait RemovePattern: Sized {
    fn remove_pattern(self, pattern: &str) -> RemovePatternIter;
}

impl RemovePattern for String {
    fn remove_pattern(self, pattern: &str) -> RemovePatternIter {
        RemovePatternIter {
            text: self,
            pattern: pattern.to_string(),
            finished: false,
        }
    }
}

// Example usage
fn main() {
    let text = "Hello, **Rust**! How are you doing **today**?".to_string();
    let pattern = "**";

    let result: String = text
        .remove_pattern(pattern)
        .collect(); // Collect the iterator into a new String
    println!("{}", result); // Output: "Hello, Rust! How are you doing today?"
}




// Test cases
#[test]
fn test_no_pattern_occurrence() {
    let text = "No pattern here.".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "No pattern here.");
}

#[test]
fn test_single_pattern_occurrence() {
    let text = "Pattern ** found.".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern  found.");
}

#[test]
fn test_multiple_pattern_occurrences() {
    let text = "Pattern ** found ** twice.".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern  found  twice.");
}

#[test]
fn test_pattern_at_beginning() {
    let text = "**Pattern at the beginning.".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern at the beginning.");
}

#[test]
fn test_pattern_at_end() {
    let text = "Pattern at the end.**".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern at the end.");
}

#[test]
fn test_empty_text() {
    let text = "".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "");
}

// TODO: stuck
#[test]
fn test_empty_pattern() {
    let text = "Testing with empty pattern.".to_string();
    let pattern = "";
    let result: String = text.clone().remove_pattern(pattern).collect();
    assert_eq!(result, text);
}

#[test]
fn test_large_text_and_pattern() {
    let text = "Large text with multiple pattern occurrences: **Pattern** **Pattern** **Pattern**.".to_string();
    let pattern = "**Pattern**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Large text with multiple pattern occurrences:   .");
}

#[test]
fn test_pattern_not_found() {
    let text = "No pattern in this text.".to_string();
    let pattern = "missing";
    let result: String = text.clone().remove_pattern(pattern).collect();
    assert_eq!(result, text);
}

#[test]
fn test_pattern_empty_text() {
    let text = "".to_string();
    let pattern = "something";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "");
}

#[test]
fn test_pattern_empty_text_and_pattern() {
    let text = "".to_string();
    let pattern = "";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "");
}

#[test]
fn test_single_character_pattern() {
    let text = "a b c d e".to_string();
    let pattern = " ";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "abcde");
}

#[test]
fn test_large_pattern() {
    let text = "aaaXbbbXcccXdddXeee".to_string();
    let pattern = "X";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "aaabbbcccdddeee");
}

#[test]
fn test_pattern_occurs_after_empty_string() {
    let text = "abc X defXghi".to_string();
    let pattern = "X";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "abc  defghi");
}

#[test]
fn test_multiple_consecutive_patterns() {
    let text = "Hello **world****today!**".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Hello worldtoday!");
}

#[test]
fn test_pattern_with_special_characters() {
    let text = "Testing with $$$ pattern $$$".to_string();
    let pattern = "$$$";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Testing with  pattern ");
}

#[test]
fn test_pattern_at_multiple_positions() {
    let text = "Pattern ** found ** multiple ** times ** in ** text.".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "Pattern  found  multiple  times  in  text.");
}

#[test]
fn test_pattern_with_digits() {
    let text = "123 ** 456 ** 789".to_string();
    let pattern = "**";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "123  456  789");
}

// TODO: enable case-sensitive?
#[test]
fn test_case_insensitive_pattern() {
    let text = "CASE ** SeNsItIvE ** PaTtErN.".to_string();
    let pattern = "sEnSiTiVe";
    let result: String = text.remove_pattern(pattern).collect();
    assert_eq!(result, "CASE ** SeNsItIvE ** PaTtErN.");
}