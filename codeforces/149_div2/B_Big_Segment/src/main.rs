#![allow(unused_imports)]
use std::collections::{HashSet, HashMap};
use std::cmp::{min,max};
use std::io;
use std::str;
 
struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitWhitespace<'static>,
}
impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self { reader, buf_str: vec![], buf_iter: "".split_whitespace() }
    }
    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            // If we have another token in this line
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse"); // calls parse::<T>() on the current token and returns it.
            }

            // If we do not have another token in the line then
            // we should go to the next line.
            self.buf_str.clear(); // empty out the buffer that holds the current line
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Failed read"); // read a new line into the buffer

            // create an iterator over the white space delimited strings of the current line
            self.buf_iter = unsafe { 
                let slice = str::from_utf8_unchecked(&self.buf_str); // makes a string from utf8 characters in the current line
                std::mem::transmute(slice.split_whitespace()) // transmutes the memory in place 
            }
        }
    }
}
 
fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n = scan.token::<u32>();
    let mut mn: (Option<u32>, Option<u32>, Option<u32>) = (None, None, None);
    let mut mx: (Option<u32>, Option<u32>, Option<u32>) = (None, None, None);
    for i in 0..n {
        let segment = (scan.token::<u32>(), scan.token::<u32>());
        
        if mn.0.is_none() {
            mn = (Some(i + 1), Some(segment.0), Some(segment.1));
            mx = (Some(i + 1), Some(segment.0), Some(segment.1));
        } else {
            if segment.0 < mn.1.unwrap() {
                mn = (Some(i + 1), Some(segment.0), Some(segment.1));
            }

            if segment.0 == mn.1.unwrap() && segment.1 > mn.2.unwrap() {
                mn = (Some(i + 1), Some(segment.0), Some(segment.1));
            }

            if segment.1 > mx.2.unwrap() {
                mx = (Some(i + 1), Some(segment.0), Some(segment.1));
            }

            if segment.1 == mx.2.unwrap() && segment.0 < mx.1.unwrap() {
                mx = (Some(i + 1), Some(segment.0), Some(segment.1));
            }
        }
    }

    if mn == mx {
        writeln!(out, "{}", mn.0.unwrap());
    } else {
        writeln!(out, "-1");
    }
}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
