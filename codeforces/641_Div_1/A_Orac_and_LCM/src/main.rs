use std::io;

fn main() {
    let mut n = String::new();
    let mut input = String::new();
    io::stdin().read_line(&mut n).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    println!("{}", solve(&s));
}

fn lcm(a: u32,b: u32) -> u32 {
    a * b / gcd(&vec![a,b])
}

fn gcd (s: &Vec<u32>) -> u32 {
    match all_same(s) {
        true => s[0],
        false => {
            let min_elem = s.iter().min().unwrap();
            gcd(&s.iter().map(|x| match x {
                j if j > min_elem => j - min_elem,
                _ => *x
            }).collect())
        }
    }
}

fn all_same (s: &Vec<u32>) -> bool {
    if s.len() <= 1 {
        return true
    }

    let first = &s[0];
    for x in s {
        if x != first {
            return false
        }
    }

    true
}

fn solve(s: &Vec<u32>) -> u32 {
    let mut x = Vec::<u32>::new();
    for (i, a) in s.iter().enumerate() {
        for b in s.iter().skip(i+1) {
            x.push(lcm(*a,*b));
        }
    }
    gcd(&x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(&vec![120, 40, 80, 120, 240, 80]),40)
    }

    #[test]
    fn test_all_same_true() {
        assert_eq!(all_same(&vec![0,0,0,0]), true)
    }

    #[test]
    fn test_all_same_false() {
        assert_eq!(all_same(&vec![0,0,0,1]), false)
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(10, 24), 120)
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(&vec![1, 1]), 1);
        assert_eq!(solve(&vec![10, 24, 40, 80]), 40);
        assert_eq!(solve(&vec![540, 648, 810, 648, 720, 540, 594, 864, 972, 648]), 54)
    }
}
