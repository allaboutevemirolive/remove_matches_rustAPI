use regex::Regex;

// Delete specific word with regex
pub fn delete_specific_words(data: &str, words_to_be_deleted: &[&str]) -> String {
    if data.is_empty() || words_to_be_deleted.is_empty() {
        return data.to_string();
    }

    let re_words = Regex::new(&format!(r"({})", words_to_be_deleted.join("|"))).unwrap();
    let mut new_data = String::new();

    for line in data.lines() {
        let line_without_brackets = re_words.replace_all(line, "");
        new_data += &line_without_brackets;
        new_data.push('\n');
    }

    new_data
}

fn main() {
    let data = "This is a sample text with unwanted words.";
    let words_to_be_deleted = ["sample", "unwanted"];

    let result = delete_specific_words(data, &words_to_be_deleted);
    println!("{}", result);
}
