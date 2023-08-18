use std::fmt;

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

        let combined_pattern = self.patterns.join("|");
        let regex = regex::Regex::new(&combined_pattern).unwrap();

        let mut result = String::new();
        let mut last_end = 0;

        for mat in regex.find_iter(self.text) {
            result.push_str(&self.text[last_end..mat.start()]);
            last_end = mat.end();
        }

        result.push_str(&self.text[last_end..]);
        result
    }
}

impl<'a> fmt::Display for PatternRemover<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.remove_patterns())
    }
}

fn main() {
    let data = "Hello, **Rust**! How are you doing **today**?\nAnother line with **pattern**.";
    let patterns = vec!["\\*\\*"]; // Double asterisk pattern

    let remover = PatternRemover::new(data, patterns);
    println!("{}", remover);
}
