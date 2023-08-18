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
        if self.text.is_empty() || self.patterns.is_empty() {
            return;
        }

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
                        std::ptr::copy_nonoverlapping(src, dest, len);
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
        self.text.shrink_to_fit(); // Minimize allocation overhead
    }

    pub fn into_text(self) -> Vec<u8> {
        self.text
    }
}

fn main() {
    let data = "Hello, **Rust**! How are you doing **today**?\nAnother line with **pattern**.";
    let patterns = vec!["**"];

    let mut remover = PatternRemover::new(data.to_string().into_bytes(), patterns);
    remover.remove_patterns();
    let result = remover.into_text();
    println!("{}", String::from_utf8_lossy(&result));
}
