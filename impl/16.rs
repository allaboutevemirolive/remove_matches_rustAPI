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
        let mut lines = self.text.lines();

        if let Some(mut line) = lines.next() {
            for pattern in &self.patterns {
                while let Some(pos) = line.find(pattern) {
                    result.push_str(&line[..pos]);
                    line = &line[pos + pattern.len()..];
                }
            }
            result.push_str(line);
        }

        for line in lines {
            result.push('\n');
            for pattern in &self.patterns {
                let mut iter = line.split(pattern);
                if let Some(part) = iter.next() {
                    result.push_str(part);
                }
                for part in iter {
                    result.push_str(part);
                }
            }
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
