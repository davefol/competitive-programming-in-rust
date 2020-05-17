use std::io;

fn main() {
    let spd = sieve(1000000);
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    let mut input = String::new();

    for _ in 0..(t.trim().parse::<u32>().unwrap()) { 
        io::stdin().read_line(&mut input);
        let numbers: Vec<u32> = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        let (n, k) = (numbers[0], numbers[1]);

        println!("{}", solve(n, k, &spd));

        input = String::new();
    }
}

fn solve(mut n: u32, k: u32, spd: &Vec<u32>) -> u32 {
    if n % 2 == 0 {
        return n + 2*k;
    } else {
        let x = spd[n as usize] + n;
        return x + 2 * (k-1)
    }
}

fn sieve(limit: usize) -> Vec<u32> {
    let mut ret: Vec<u32> = vec![0; limit+1];
    
    for i in 2..limit {
        if ret[i] == 0 {
            ret[i] = i as u32;
            for j in ((i+i)..limit).step_by(i) {
                if ret[j] == 0 {
                    ret[j] = i as u32;
                }
            }
        }
    }

    ret
    
}
