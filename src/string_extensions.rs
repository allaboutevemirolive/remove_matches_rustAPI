use core::iter::from_fn;
use core::str::pattern::Pattern;
use std::str::pattern::Searcher;

pub trait StringExtensions {
    fn remove_matches_test<P>(&mut self, pat: P)
    where
        P: for<'x> Pattern<'x>;
}

impl StringExtensions for String {
    fn remove_matches_test<P>(&mut self, pat: P)
    where
        P: for<'x> Pattern<'x>,
    {
        // Collect the indices of the matched patterns to be removed.
        let rejections: Vec<(usize, usize)> = {
            let mut searcher = pat.into_searcher(self);
            let mut front = 0;
            let rejections: Vec<_> = from_fn(|| {
                let (start, end) = searcher.next_match()?;
                let prev_front = front;
                front = end;
                Some((prev_front, start))
            })
            .collect();
            rejections
                .into_iter()
                .chain(core::iter::once((front, self.len()))).collect()
        };

        // Remove the matched patterns from the string using `retain`.
        let mut len = 0;
        let bytes: &mut Vec<u8> = unsafe { self.as_mut_vec() };
        for (start, end) in rejections {
            let count = end - start;
            if start != len {
                bytes.copy_within(start..end, len);
            }
            len += count;
        }
        bytes.truncate(len);
    }
}


