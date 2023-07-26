
#![feature(pattern)]

pub mod string_extensions;

#[cfg(test)]
mod tests {
    // Import the extension trait
    use super::string_extensions::StringExtensions;

    // Your test cases
    #[test]
    #[cfg(feature = "string_remove_matches")]
    fn test_remove_no_matches() {
        let mut s = String::from("Hello, World!");
        s.remove_matches_test("foo");
        println!("{}", s);        
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    #[cfg(feature = "string_remove_matches")]
    fn test_remove_single_match() {
        let mut s = String::from("Hello, World!");
        s.remove_matches_test("World");
        println!("{}", s);  
        assert_eq!(s, "Hello, !");
    }

    #[test]
    #[cfg(feature = "string_remove_matches")]
    fn test_remove_multiple_matches() {
        let mut s = String::from("aaabbbccc");
        s.remove_matches_test("b");
        println!("{}", s);  
        assert_eq!(s, "aaaccc");
    }

    #[test]
    #[cfg(feature = "string_remove_matches")]
    fn test_remove_full_string_matches() {
        let mut s = String::from("abcabcabc");
        s.remove_matches_test("abc");
        println!("{}", s);  
        assert_eq!(s, "");
    }

    #[test]
    #[cfg(feature = "string_remove_matches")]
    fn test_remove_overlapping_matches() {
        let mut s = String::from("ababab");
        s.remove_matches_test("aba");
        println!("{}", s);  
        assert_eq!(s, "bab");
    }

    #[test]
    #[cfg(feature = "string_remove_matches")]
    fn test_remove_matches() {
        let mut s = "abc".to_string();

        s.remove_matches_test('b');
        assert_eq!(s, "ac");
        s.remove_matches_test('b');
        assert_eq!(s, "ac");

        let mut s = "abcb".to_string();

        s.remove_matches_test('b');
        assert_eq!(s, "ac");

        let mut s = "ศไทย中华Việt Nam; foobarศ".to_string();
        s.remove_matches_test('ศ');
        assert_eq!(s, "ไทย中华Việt Nam; foobar");

        let mut s = "".to_string();
        s.remove_matches_test("");
        assert_eq!(s, "");

        let mut s = "aaaaa".to_string();
        s.remove_matches_test('a');
        assert_eq!(s, "");
    }
}
