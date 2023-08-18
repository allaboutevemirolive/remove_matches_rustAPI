use regex::Regex;

fn main() {
    let text = "Hello, **Rust**! How are you doing **today**?\nAnother line with **pattern**.";
    let pattern = r"\*\*"; // Double asterisk pattern

    let mut result = String::new();
    let mut last_end = 0;
    for mat in Regex::new(pattern).unwrap().find_iter(text) {
        let non_match = &text[last_end..mat.start()];
        result.push_str(non_match);
        last_end = mat.end();
    }
    result.push_str(&text[last_end..]);

    println!("{}", result);
}
