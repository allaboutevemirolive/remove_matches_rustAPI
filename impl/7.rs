pub struct RemovePatternIter<T, U>
where
    T: Iterator<Item = U>,
    U: AsRef<str>,
{
    text: T,
    pattern: String,
    finished: bool,
}

impl<T, U> Iterator for RemovePatternIter<T, U>
where
    T: Iterator<Item = U> + Clone, // Add Clone trait here
    U: AsRef<str>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        // Handle the case when the pattern is an empty string
        if self.pattern.is_empty() {
            self.finished = true;
            return Some(self.text.clone().map(|s| s.as_ref().to_string()).collect::<String>());
        }

        let mut joined_text: String = self.text.clone().map(|s| s.as_ref().to_string()).collect(); // Make joined_text mutable
        while let Some(pos) = joined_text.find(&self.pattern) {
            let non_match = joined_text[..pos].to_string();
            // Skip pattern and update joined_text
            joined_text = joined_text[pos + self.pattern.len()..].to_string();
            joined_text.insert_str(0, &non_match);
        }

        self.finished = true;
        Some(joined_text)
    }
}

pub trait RemovePattern<T, U>: Sized
where
    T: Iterator<Item = U>,
    U: AsRef<str>,
{
    fn remove_pattern(self, pattern: String) -> RemovePatternIter<T, U>;
}

impl<T: ?Sized, U> RemovePattern<T, U> for T
where
    T: Iterator<Item = U> + Clone, // Add Clone trait here
    U: AsRef<str>,
{
    fn remove_pattern(self, pattern: String) -> RemovePatternIter<T, U> {
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

    // Convert the text into an iterator over words
    let text_iter = text.split_whitespace();

    let result: String = text_iter.remove_pattern(pattern.to_string()).collect();
    println!("{}", result); // Output: "Hello, Rust! How are you doing today?"
}

