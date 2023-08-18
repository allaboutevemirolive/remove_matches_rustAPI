#![feature(allocator_api)]

pub struct PatternRemover {
    pub text: Vec<u8>,
    pub patterns: Vec<Vec<u8>>,
}

impl PatternRemover {
    pub fn new(text: Vec<u8>, patterns: Vec<&str>) -> Self {
        let patterns = patterns.iter().map(|&p| p.as_bytes().to_vec()).collect();
        Self { text, patterns }
    }

    pub fn remove_patterns(&mut self) {
        let mut new_text = Vec::new();
        let text_len = self.text.len();
        let mut read_pos = 0;

        while read_pos < text_len {
            let mut matched = false;

            for pattern in &self.patterns {
                if let Some(pos) = self.find_pattern(pattern, read_pos) {
                    new_text.extend_from_slice(&self.text[read_pos..pos]);
                    read_pos = pos + pattern.len();
                    matched = true;
                    break;
                }
            }

            if !matched {
                new_text.push(self.text[read_pos]);
                read_pos += 1;
            }
        }

        self.text = new_text;
    }

    fn build_prefix_function(pattern: &[u8]) -> Vec<usize> {
        let pattern_len = pattern.len();
        let mut prefix_function = vec![0; pattern_len];
        let mut length = 0;
        let mut i = 1;

        while i < pattern_len {
            if pattern[i] == pattern[length] {
                length += 1;
                prefix_function[i] = length;
                i += 1;
            } else {
                if length != 0 {
                    length = prefix_function[length - 1];
                } else {
                    prefix_function[i] = 0;
                    i += 1;
                }
            }
        }

        prefix_function
    }

    fn find_pattern(&self, pattern: &[u8], start_pos: usize) -> Option<usize> {
        let text_len = self.text.len();
        let pattern_len = pattern.len();
        let prefix_function = Self::build_prefix_function(pattern);
        let mut i = start_pos;
        let mut j = 0;

        while i < text_len {
            if pattern[j] == self.text[i] {
                j += 1;
                i += 1;
                if j == pattern_len {
                    return Some(i - j);
                }
            } else if j > 0 {
                j = prefix_function[j - 1];
            } else {
                i += 1;
            }
        }

        None
    }

    pub fn into_text(self) -> Vec<u8> {
        self.text
    }
}

pub struct MyString {
    pub vec: Vec<u8>,
}

impl MyString {
    pub fn new(initial_text: &[u8]) -> Self {
        Self {
            vec: initial_text.to_vec(),
        }
    }

    pub fn remove_matches(&mut self, patterns: Vec<&str>) {
        let mut remover = PatternRemover::new(Vec::new(), patterns);
        std::mem::swap(&mut self.vec, &mut remover.text);
        remover.remove_patterns();
        self.vec = remover.into_text();
    }
}

fn main() {
    let mut my_string = MyString::new("Hello, bugs and beautiful world!".as_bytes());

    println!("Before: {:?}", String::from_utf8_lossy(&my_string.vec));
    let patterns_to_remove = vec!["beautiful"];
    my_string.remove_matches(patterns_to_remove);
    println!("After: {:?}", String::from_utf8_lossy(&my_string.vec));
}
