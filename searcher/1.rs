impl<'a, C: MultiCharEq> Pattern<'a> for MultiCharEqPattern<C> {
    type Searcher = MultiCharEqSearcher<'a, C>;

    #[inline]
    fn into_searcher(self, haystack: &'a str) -> MultiCharEqSearcher<'a, C> {
        MultiCharEqSearcher { haystack, char_eq: self.0, char_indices: haystack.char_indices() }
    }
}