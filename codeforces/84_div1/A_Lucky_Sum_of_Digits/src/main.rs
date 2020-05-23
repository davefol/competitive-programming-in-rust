// This solution is too slow. There is a simple number theory solution available. 

#![allow(unused_imports)]
use std::collections::{HashSet, HashMap, VecDeque};
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
    let root = Node::new(n, "".to_string());
    let mut queue = VecDeque::<Node>::new();
    let mut visited = HashSet::<u32>::new();
    queue.push_back(root);

    let mut answer_found = false;

    while !queue.is_empty() {
        if let Some(current) = queue.pop_front() {

            // Answer found
            if current.value == 7 || current.value == 4 {
                writeln!(out, "{}{}", current.path, current.value);
                answer_found = true;
                break;
            }
            
            visited.insert(current.value);

            for child in current.children() {
                if !visited.contains(&child.value) {
                    queue.push_back(child);
                }
            }
        }
    }
    if !answer_found {
        writeln!(out, "-1");
    }
}

struct Node {
    value: u32,
    path: String
}

impl Node {
    fn new(value: u32, path: String) -> Self {
        Self {value, path}
    }

    fn children(&self) -> VecDeque<Node> {
        let mut s = VecDeque::<Node>::new();
        let mut bad_numbers = HashSet::<u32>::new();
        
        bad_numbers.insert(1);
        bad_numbers.insert(2);
        bad_numbers.insert(3);
        bad_numbers.insert(5);
        bad_numbers.insert(6);

        if self.value > 5 && !bad_numbers.contains(&(self.value - 4)) {
            s.push_back(Self::new(self.value - 4, format!("{}{}", self.path, "4")));
        }
        if self.value > 8 && !bad_numbers.contains(&(self.value - 7)) {
            s.push_back(Self::new(self.value - 7, format!("{}{}", self.path, "7")));
        }
        s
    }
}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
