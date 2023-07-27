    #[cfg(not(no_global_oom_handling))]
    #[unstable(feature = "string_remove_matches", reason = "new API", issue = "72826")]
    pub fn remove_matches<'a, P>(&'a mut self, pat: P)
    where
        P: for<'x> Pattern<'x>,
    {
        use core::str::pattern::Searcher;

        let rejections = {
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
                .chain(core::iter::once((front, self.len())))
        };

        let mut len = 0;
        let ptr = self.vec.as_mut_ptr();

        for (start, end) in rejections {
            let count = end - start;
            if start != len {
                
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