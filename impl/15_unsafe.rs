pub struct PatternRemover<'a> {
    text: &'a str,
    patterns: Vec<&'a str>,
}

impl<'a> PatternRemover<'a> {
    pub fn new(text: &'a str, patterns: Vec<&'a str>) -> Self {
        Self { text, patterns }
    }

    pub fn remove_patterns(&self) -> String {
        if self.text.is_empty() || self.patterns.is_empty() {
            return self.text.to_string();
        }

        let mut result = String::new();

        for line in self.text.lines() {
            let mut modified_line = String::with_capacity(line.len()); // Preallocate space

            for c in line.chars() {
                modified_line.push(c);
            }

            for pattern in &self.patterns {
                // Use unsafe to modify the string in place
                unsafe {
                    let mut start = 0;
                    while let Some(pos) = modified_line[start..].find(pattern) {
                        let real_pos = start + pos;
                        modified_line
                            .as_mut_vec()
                            .splice(real_pos..real_pos + pattern.len(), Vec::new());
                        start = real_pos;
                    }
                }
            }

            result.push_str(&modified_line);
            result.push('\n');
        }

        result
    }
}

impl<'a> std::fmt::Display for PatternRemover<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.remove_patterns())
    }
}

fn main() {
    let data = "Hello, **Rust**! How are you doing **today**?\nAnother line with **pattern**.";
    let patterns = vec!["**"]; // Double asterisk pattern

    let remover = PatternRemover::new(data, patterns);
    println!("{}", remover);
}
