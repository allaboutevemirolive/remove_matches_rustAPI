```rust
pub fn remove_matches_inplace(text: &mut String, pat: &str) {
    let pat_len = pat.len();
    let mut start = 0;
    let mut end = text.len();

    while let Some(pos) = text[start..end].find(pat) {
        let match_start = start + pos;
        let match_end = match_start + pat_len;

        // Replace the matched pattern with an empty string
        text.replace_range(match_start..match_end, "");

        // Move the start position after the matched pattern
        start = match_start;
        // Move the end position back by the length of the pattern
        end -= pat_len;
    }
}


fn main() {
    let mut text = String::from("Hello, **Rust**! How are you doing **today**?");
    let pattern = "**";

    remove_matches_inplace(&mut text, pattern);
    println!("{}", text); // Output: "Hello, Rust! How are you doing today?"
}
```