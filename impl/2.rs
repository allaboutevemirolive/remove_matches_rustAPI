#[cfg(not(no_global_oom_handling))]
#[unstable(feature = "string_remove_matches", reason = "new API", issue = "72826")]
pub fn remove_matches<'a, P>(&'a mut self, pat: P)
where
    P: for<'x> Pattern<'x>,
{
    use core::str::pattern::Searcher;

    let mut searcher = pat.into_searcher(self);
    // Per Searcher::next:
    //
    // A Match result needs to contain the whole matched pattern,
    // however Reject results may be split up into arbitrary many
    // adjacent fragments. Both ranges may have zero length.
    //
    // In practice the implementation of Searcher::next_match tends to
    // be more efficient, so we use it here and do some work to invert
    // matches into rejections since that's what we want to copy below.

    // Using `Iterator::try_fold` to build the `rejections` vector without unnecessary cloning.
    let mut rejections = Vec::new();
    let mut front = 0;
    while let Some((start, end)) = searcher.next_match() {
        rejections.push((front, start));
        front = end;
    }
    rejections.push((front, self.len()));

    let mut len = 0;
    let ptr = self.vec.as_mut_ptr();

    for (start, end) in rejections {
        let count = end - start;
        if start != len {
            // SAFETY: per Searcher::next:
            //
            // The stream of Match and Reject values up to a Done will
            // contain index ranges that are adjacent, non-overlapping,
            // covering the whole haystack, and laying on utf8
            // boundaries.
            unsafe {
                ptr::copy(ptr.add(start), ptr.add(len), count);
            }
        }
        len += count;
    }

    unsafe {
        self.vec.set_len(len);
    }
}
