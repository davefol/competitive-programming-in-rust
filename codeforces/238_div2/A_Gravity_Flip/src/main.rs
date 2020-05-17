use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = String::new();
    io::stdin().read_line(&mut  buffer).unwrap();
    let mut a: Vec<u32> = buffer.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    a.sort_unstable();
    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));

}
