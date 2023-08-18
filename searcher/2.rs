fn into_searcher(self, haystack: &'a str) -> Self::Searcher {
    let mut utf8_encoded = [0; 4];
    let utf8_size = self.encode_utf8(&mut utf8_encoded).len();
    CharSearcher {
        haystack,
        finger: 0,
        finger_back: haystack.len(),
        needle: self,
        utf8_size,
        utf8_encoded,
    }
}