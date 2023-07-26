#![allow(unused)]
#![feature(string_remove_matches)]
fn main() {
    let mut s = String::from("banana");
    s.remove_matches("ana");
    assert_eq!("bna", s);


}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remove_matches() {
        let mut s = "abc".to_string();

        s.remove_matches('b');
        assert_eq!(s, "ac");
        s.remove_matches('b');
        assert_eq!(s, "ac");

        let mut s = "abcb".to_string();

        s.remove_matches('b');
        assert_eq!(s, "ac");

        let mut s = "ศไทย中华Việt Nam; foobarศ".to_string();
        s.remove_matches('ศ');
        assert_eq!(s, "ไทย中华Việt Nam; foobar");

        let mut s = "".to_string();
        s.remove_matches("");
        assert_eq!(s, "");

        let mut s = "aaaaa".to_string();
        s.remove_matches('a');
        assert_eq!(s, "");
    }
}
