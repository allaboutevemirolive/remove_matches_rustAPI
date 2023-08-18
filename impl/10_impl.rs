// The iterator-based approach can be efficient
// for small patterns or texts with few occurrences of the
// pattern. However, if the pattern or text is large, or if
// there are many occurrences of the pattern, it may not be
// as efficient as other algorithms or regular expressions.

mod main_test;

// This code iterate over a text in a line.
// How about text in multiple line?
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
    fn remove_pattern(self, pattern: &'a str) -> RemovePatternIter<'a>;
}

impl<'a> RemovePattern<'a> for &'a str {
    fn remove_pattern(self, pattern: &'a str) -> RemovePatternIter<'a> {
        RemovePatternIter {
            text: self,
            pattern,
            finished: false,
        }
    }
}

// Example usage
fn main() {
    let text = "Hello, **Rust**! How are you doing **today**?";
    let pattern = "**";

    let result: String = text.remove_pattern(pattern).collect(); // Collect the iterator into a new String
    println!("{}", result); // Output: "Hello, Rust! How are you doing today?"
}
