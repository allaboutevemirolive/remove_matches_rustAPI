#![feature(pattern)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]

use std::ptr;
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
                if write_pos != read_pos {
                    unsafe {
                        let src = self.text.as_ptr().add(read_pos);
                        let dest = self.text.as_mut_ptr().add(write_pos);
                        let len = 1; // Number of bytes to copy
                        ptr::copy_nonoverlapping(src, dest, len);
                    }
                }
                write_pos += 1;
                read_pos += 1;
            } else {
                while read_pos < self.text.len() && self.patterns.iter().any(|pattern| self.text[read_pos..].starts_with(pattern)) {
                    read_pos += 1;
                }
            }
        }

        self.text.truncate(write_pos);
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
        vec: Vec::from("Hello, beautiful world!".as_bytes()),
    };

    println!("Before: {:?}", String::from_utf8_lossy(&my_string.vec));
    let patterns_to_remove = vec!["beautiful"];
    my_string.remove_matches(patterns_to_remove);
    println!("After: {:?}", String::from_utf8_lossy(&my_string.vec));
}
