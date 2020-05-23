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
 
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    x: u32,
    y: u32
}

impl Node {
    fn new(x:u32, y:u32) -> Self {
        Self {x, y}
    }
}

fn dfs (node: Node, x_neighbors: HashMap<u32, Vec<Node>>, y_neighbors: HashMap<u32, Vec<Node>>, visited: HashSet<Node>) {

}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n = scan.token::<usize>();
    let mut x_neighbors = HashMap::<u32, Vec<Node>>::new();
    let mut y_neighbors = HashMap::<u32, Vec<Node>>::new();
    let mut nodes = Vec::<Node>::new();
    for _ in 0..n {
        let node = Node::new(scan.token::<u32>(), scan.token::<u32>());
        nodes.push(node);
        x_neighbors.entry(node.x).or_insert(Vec::<Node>::new()).push(node);
        y_neighbors.entry(node.y).or_insert(Vec::<Node>::new()).push(node);
    }
    let mut visited = HashSet::<Node>::new();
    let mut components = 0;
    for node in nodes {
        if !visited.contains(&node) {
            components += 1;
            let mut stack = Vec::<Node>::new();
            stack.push(node);
            while !stack.is_empty() {
                let nd = stack.pop().unwrap();
                if visited.contains(&nd) {
                    continue;
                } else {
                    visited.insert(nd);
                    for neighbor in x_neighbors.get(&nd.x).unwrap().iter().chain(y_neighbors.get(&nd.y).unwrap().iter()) {
                        stack.push(*neighbor);
                    }
                }
            }
        }
    }
    writeln!(out, "{}", components - 1);


}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
