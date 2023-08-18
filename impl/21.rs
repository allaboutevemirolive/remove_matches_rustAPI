#![feature(pattern)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]

use std::str::pattern::{Pattern, Searcher};
use std::ptr;
use std::vec::Vec;

struct MyString {
    vec: Vec<u8>,
}

impl MyString {
    pub fn remove_matches<P>(&mut self, pat: P)
    where
        P: for<'x> Pattern<'x>,
    {
        let vec_len = self.vec.len();
        let ptr = self.vec.as_mut_ptr();

        let rejections = {
            let vec_as_str = unsafe {
                std::str::from_utf8_unchecked(&self.vec)
            };

            let mut searcher = pat.into_searcher(vec_as_str);

            let mut front = 0;
            let mut rejections = Vec::new();
            while let Some((start, end)) = searcher.next_match() {
                rejections.push((front, start));
                front = end;
            }
            rejections.push((front, vec_len));
            rejections
        };

        let mut len = 0;

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
