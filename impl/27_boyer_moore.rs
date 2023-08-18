#![feature(allocator_api)]

use std::vec::Vec;

pub struct PatternRemover {
    text: Vec<u8>,
    patterns: Vec<Vec<u8>>,
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

    fn find_pattern(&self, pattern: &[u8], start_pos: usize) -> Option<usize> {
        let text_len = self.text.len();
        let pattern_len = pattern.len();
        let mut i = start_pos + pattern_len - 1;

        while i < text_len {
            let mut j = pattern_len - 1;
            let mut k = i;

            while j > 0 && self.text[k] == pattern[j] {
                j -= 1;
                k -= 1;
            }

            if j == 0 && self.text[k] == pattern[j] {
                return Some(k);
            }

            i += 1;
        }

        None
    }

    pub fn into_text(self) -> Vec<u8> {
        self.text
    }
}

struct MyString {
    vec: Vec<u8>,
}

impl MyString {
    pub fn remove_matches(&mut self, patterns: Vec<&str>) {
        let mut remover = PatternRemover::new(Vec::new(), patterns);
        std::mem::swap(&mut self.vec, &mut remover.text);
        remover.remove_patterns();
        self.vec = remover.into_text();
    }
}

fn main() {
    let mut my_string = MyString {
        vec: Vec::from("Hello, bugs and beautiful world!".as_bytes()),
    };

    println!("Before: {:?}", String::from_utf8_lossy(&my_string.vec));
    let patterns_to_remove = vec!["beautiful"];
    my_string.remove_matches(patterns_to_remove);
    println!("After: {:?}", String::from_utf8_lossy(&my_string.vec));
}
