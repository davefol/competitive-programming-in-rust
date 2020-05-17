use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
    buf.clear();
    io::stdin().read_line(&mut buf);
    let s: Vec<u32> = buf.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut mx = 1;
    for i in 0..s.len() {
        let mut curr = 0;
        let mut mx_height = s[i];
        for j in i..s.len() {
            if s[j] <= mx_height {
                mx_height = s[j];
                curr += 1
            } else {
                break;
            }
        }
        mx_height = s[i];
        for j in (0..i).rev() {
            if s[j] <= mx_height {
                mx_height = s[j];
                curr += 1
            } else {
                break;
            }
        }
        mx = u32::max(mx, curr);
    }
    println!("{}", mx);
}
