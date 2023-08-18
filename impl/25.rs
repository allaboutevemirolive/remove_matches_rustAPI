use std::vec::Vec;

struct PatternRemover<'a> {
    text: &'a mut Vec<u8>,
    patterns: Vec<Vec<u8>>,
}

impl<'a> PatternRemover<'a> {
    pub fn new(text: &'a mut Vec<u8>, patterns: Vec<Vec<u8>>) -> Self {
        Self { text, patterns }
    }

    pub fn remove_patterns(&mut self) {
        let mut write_pos = 0;
        let mut read_pos = 0;

        while read_pos < self.text.len() {
            let mut pattern_matched = false;

            for pattern in &self.patterns {
                if self.text[read_pos..].starts_with(pattern) {
                    read_pos += pattern.len();
                    pattern_matched = true;
                    break;
                }
            }

            if !pattern_matched {
                let count = read_pos - write_pos;
                if count > 0 {
                    self.text.copy_within(write_pos..read_pos, write_pos);
                }
                write_pos += count + 1; // +1 to skip the current byte
                read_pos += 1;
            } else {
                while read_pos < self.text.len() && self.patterns.iter().any(|pattern| self.text[read_pos..].starts_with(pattern)) {
                    read_pos += 1;
                }
            }
        }

        let count = read_pos - write_pos;
        if count > 0 {
            self.text.copy_within(write_pos..read_pos, write_pos);
        }

        self.text.truncate(write_pos + count);
    }
}

struct MyString {
    vec: Vec<u8>,
}

impl MyString {
    pub fn remove_matches(&mut self, patterns: Vec<Vec<u8>>) {
        let mut remover = PatternRemover::new(&mut self.vec, patterns);
        remover.remove_patterns();
    }
}

fn main() {
    let mut my_string = MyString {
        vec: Vec::from("Hello, beautiful world!".as_bytes()),
    };

    println!("Before: {:?}", String::from_utf8_lossy(&my_string.vec));
    let patterns_to_remove: Vec<Vec<u8>> = vec![Vec::from("beautiful")];
    my_string.remove_matches(patterns_to_remove);
    println!("After: {:?}", String::from_utf8_lossy(&my_string.vec));
}
