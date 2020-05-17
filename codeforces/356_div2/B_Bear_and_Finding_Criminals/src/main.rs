use std::io;
fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n_a: Vec<usize> = buffer.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let (_, a) = (n_a[0], n_a[1] - 1);
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let s: Vec<u32> = buffer.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let i: usize = a;
    let mut d: usize = 1;
    let mut ans: u32 = 0;
    if s[a] == 1 { ans += 1};
    loop {
        let left_idx = i.checked_sub(d);
        let right_idx = i + d;

        let left = match left_idx {
            Some(x) => Some(s[x]),
            None => None
        };

        let right = match right_idx {
            x if x < s.len() => Some(s[x]),
            _ => None
        };

        ans += match (left, right) {
            (Some(1), Some(1)) => 2,
            (None, Some(1)) => 1,
            (Some(1), None) => 1,
            _ => 0
        };

        if left.is_none() && right.is_none() {
            break;
        }

        d += 1
    }
    println!("{}", ans);
}
