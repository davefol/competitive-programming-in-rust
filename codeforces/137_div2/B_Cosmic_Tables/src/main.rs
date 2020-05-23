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
    let rows = scan.token::<usize>();
    let columns = scan.token::<usize>();
    let queries = scan.token::<u32>();

    let mut table: Vec<Vec<u32>> = vec![];
    let mut rowidx = (0..rows).collect::<Vec<usize>>();
    let mut colidx = (0..columns).collect::<Vec<usize>>();
    
    // populate table
    for _ in 0..rows {
        let mut row = vec![];
        for _ in 0..columns {
            row.push(scan.token::<u32>());
        }
        table.push(row);
    }

    for _ in 0..queries {
        match scan.token::<char>() {
            'g' => {
                let row = scan.token::<usize>();
                let column = scan.token::<usize>();
                writeln!(out, "{}", table[rowidx[row-1]][colidx[column-1]]);
            },
            'r' => {
                let a = scan.token::<usize>();
                let b = scan.token::<usize>();
                rowidx.swap(a-1,b-1);
            },
            'c' => {
                let a = scan.token::<usize>();
                let b = scan.token::<usize>();
                colidx.swap(a-1,b-1);
            },
            _ => unreachable!()
        }
    }



}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
