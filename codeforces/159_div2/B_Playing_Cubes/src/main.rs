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
 
#[derive(PartialEq)]
enum Cube {
    Red,
    Blue
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let mut red = scan.token::<u32>();
    let mut blue = scan.token::<u32>();
    let mut last_cube: Option<Cube> = None;
    let mut petya = true;
    let mut petya_score: u32 = 0;
    let mut vasya_score: u32 = 0;

    if blue < red {
        if blue % 2 != 0 {
            last_cube = Some(Cube::Blue);
            blue -= 1;
        } else {
            last_cube = Some(Cube::Red);
            red -= 1;
        }
    } else if red < blue {
        if red % 2 != 0 {
            last_cube = Some(Cube::Red);
            red -= 1;
        } else {
            last_cube = Some(Cube::Blue);
            blue -= 1;
        }
    }

    while red > 0 || blue > 0 {
        let desired_color = {
            if petya {
                match last_cube {
                    Some(Cube::Red) => Cube::Red,
                    Some(Cube::Blue) => Cube::Blue,
                    None => Cube::Red
                }
            } else {
                match last_cube {
                    Some(Cube::Red) => Cube::Blue,
                    Some(Cube::Blue) => Cube::Red,
                    None => Cube::Red
                }
            }
        };
        
        let current_cube = {
            if desired_color == Cube::Red && red > 0 {
                red -= 1;
                Cube::Red
            } else if desired_color == Cube::Blue && blue > 0 {
                blue -=1;
                Cube::Blue
            } else if blue > 0 && red == 0 {
                blue -= 1;
                Cube::Blue
            } else {
                red -= 1;
                Cube::Red
            }
        };

        if let Some(last_cube) = last_cube {
            if last_cube == current_cube {
                petya_score += 1;
            } else {
                vasya_score += 1;
            }
        }

        last_cube = Some(current_cube);

        petya = !petya;
    }

    writeln!(out, "{} {}", petya_score, vasya_score);
}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
