#![allow(unused_imports)]
use std::collections::{HashSet, HashMap};
use std::cmp::{min,max};
use std::io;
use std::str;
use std::fs::File;
use std::io::BufReader;
 
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
    let mut boys = scan.token::<u32>();
    let mut girls = scan.token::<u32>();

    let mut s: Vec<char> = vec![];

    let mut last: char = 'G';

    if boys > girls {
        s.push('B');
        last = 'B';
        boys -= 1;

    } else {
        s.push('G');
        last = 'G';
        girls -= 1;
    }

    while boys > 0 || girls > 0 {
        if girls == 0 {
            s.push('B');
            last = 'B';
            boys -= 1;
        } else if boys == 0 {
            s.push('G');
            last = 'G';
            girls -= 1;
        } else if last == 'G' {
            s.push('B');
            last = 'B';
            boys -= 1;
        } else {
            s.push('G');
            last = 'G';
            girls -= 1;
        }
    }

    writeln!(out, "{}", s.iter().collect::<String>());
}
 
fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let mut scan = Scanner::new(input);

    let output = File::create("output.txt").unwrap();
    let mut out = io::BufWriter::new(output);

    solve(&mut scan, &mut out);
}
