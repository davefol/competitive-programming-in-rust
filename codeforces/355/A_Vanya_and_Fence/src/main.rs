use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n_h: Vec<u32> = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let (_, h) = (n_h[0], n_h[1]);

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<u32> = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    let mut ans = 0;
    for i in a {
        if i > h {
            ans += 2;
        } else {
            ans += 1;
        }
    }
    
    println!("{}", ans);
}
