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
    let n = scan.token::<i64>();
    let mut a = vec![];
    for _ in 0..n {
        a.push(scan.token::<i64>());
    }

    a.sort_unstable();

    let mut dist: u64 = 0;

    for i in 0..n {
        dist += ((a[i as usize]) - (i as i64 + 1)).abs() as u64;
    }

    writeln!(out, "{}", dist);
}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
