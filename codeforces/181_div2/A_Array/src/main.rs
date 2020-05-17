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
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
 
fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n = scan.token::<usize>();
    let mut s = vec![];
    let mut neg = vec![];
    let mut pos = vec![];
    let mut zero = vec![];

    for _ in 0..n {
        let i = scan.token::<i32>();
        if i > 0 {
            pos.push(i);
        } else if i == 0 {
            zero.push(i);
        } else {
            s.push(i);
        }
    }

    neg.push(s.pop().unwrap());
    
    if s.len() > 0 && s.len() % 2 != 0 {
        zero.push(s.pop().unwrap());
    }

    pos.append(&mut s);

    writeln!(out, "{} {}", neg.len(), neg.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    writeln!(out, "{} {}", pos.len(), pos.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    writeln!(out, "{} {}", zero.len(), zero.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
