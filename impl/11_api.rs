#![feature(pattern)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]

use std::str::pattern::Pattern;
use std::ptr;
use std::iter::from_fn;
use std::vec::Vec;

struct MyString {
    vec: Vec<u8>,
}

impl MyString {
    pub fn remove_matches<'a, P>(&'a mut self, pat: P)
    where
        P: for<'x> Pattern<'x>,
    {
        use core::str::pattern::Searcher;

        let vec_as_str = std::str::from_utf8(&self.vec).expect("Invalid UTF-8 in vec");

        let rejections = {
            let mut searcher = pat.into_searcher(vec_as_str);

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
                .chain(core::iter::once((front, self.vec.len())))
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
}

fn main() {
    let mut my_string = MyString {
        vec: Vec::from("Hello, beautiful world!".as_bytes()),
    };

    println!("Before: {:?}", String::from_utf8_lossy(&my_string.vec));
    let pattern_to_remove = "beautiful";
    my_string.remove_matches(pattern_to_remove);
    println!("After: {:?}", String::from_utf8_lossy(&my_string.vec));
}
