
// Working for string slice.
// - avoid memory allocation 
pub struct RemovePatternIter<'a> {
    text: &'a str,
    pattern: &'a str,
}

impl<'a> Iterator for RemovePatternIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }

        if let Some(pos) = self.text.find(self.pattern) {
            let non_match = &self.text[..pos];
            self.text = &self.text[pos + self.pattern.len()..];
            Some(non_match)
        } else {
            let remaining = self.text;
            self.text = "";
            Some(remaining)
        }
    }
}

pub trait RemovePattern<'a>: Sized {
    fn remove_pattern(self, pattern: &'a str) -> RemovePatternIter<'a>;
}

impl<'a> RemovePattern<'a> for &'a str {
    fn remove_pattern(self, pattern: &'a str) -> RemovePatternIter<'a> {
        RemovePatternIter {
            text: self,
            pattern,
        }
    }
}

// Example usage
fn main() {
    let text = "Hello, **Rust**! How are you doing **today**?";
    let pattern = "**";

    let result: String = text
        .remove_pattern(pattern)
        .collect(); // Collect the iterator into a new String
    println!("{}", result); // Output: "Hello, Rust! How are you doing today?"
}
