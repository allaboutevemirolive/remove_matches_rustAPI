#![feature(string_remove_matches)]

fn main() { }

#[test]
fn test_remove_matches() {

    // test_single_pattern_occurrence
    let mut s = "abc".to_string();
    s.remove_matches('b');
    assert_eq!(s, "ac");
    // repeat_test_single_pattern_occurrence
    s.remove_matches('b');
    assert_eq!(s, "ac");

    // test_single_character_pattern
    let mut s = "abcb".to_string();
    s.remove_matches('b');
    assert_eq!(s, "ac");

    // test_pattern_with_special_characters
    let mut s = "ศไทย中华Việt Nam; foobarศ".to_string();
    s.remove_matches('ศ');
    assert_eq!(s, "ไทย中华Việt Nam; foobar");

    // test_pattern_empty_text_and_pattern
    let mut s = "".to_string();
    s.remove_matches("");
    assert_eq!(s, "");

    // test_pattern_empty_text
    let mut s = "".to_string();
    s.remove_matches("something");
    assert_eq!(s, "");

    // test_empty_pattern
    let mut s = "Testing with empty pattern.".to_string();
    s.remove_matches("");
    assert_eq!(s, "Testing with empty pattern.");

    // test_multiple_consecutive_patterns_1
    let mut s = "aaaaa".to_string();
    s.remove_matches('a');
    assert_eq!(s, "");

    // test_multiple_consecutive_patterns_2
    let mut s = "Hello **world****today!**".to_string();
    s.remove_matches("**");
    assert_eq!(s, "Hello worldtoday!");

    // test_case_insensitive_pattern
    let mut s = "CASE ** SeNsItIvE ** PaTtErN.".to_string();
    s.remove_matches("sEnSiTiVe");
    assert_eq!(s, "CASE ** SeNsItIvE ** PaTtErN.");

    // test_pattern_with_digits
    let mut s = "123 ** 456 ** 789".to_string();
    s.remove_matches("**");
    assert_eq!(s, "123  456  789");

    // test_pattern_occurs_after_empty_string
    let mut s = "abc X defXghi".to_string();
    s.remove_matches("X");
    assert_eq!(s, "abc  defghi");

    // test_large_pattern
    let mut s = "aaaXbbbXcccXdddXeee".to_string();
    s.remove_matches("X");
    assert_eq!(s, "aaabbbcccdddeee");

    // test_pattern_at_multiple_positions
    let mut s = "Pattern ** found ** multiple ** times ** in ** text.".to_string();
    s.remove_matches("**");
    assert_eq!(s, "Pattern  found  multiple  times  in  text.");

}
