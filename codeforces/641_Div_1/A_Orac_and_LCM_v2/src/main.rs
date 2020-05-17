use std::io;
use std::collections::HashMap;

fn main() {
    // compute primes up to 200,001
    let primes = sieve(200000);

    // factorize each number of input
    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let s: Vec<u32> = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

    let mut prime_count = HashMap::<u32, Vec<u32>>::new();

    for x in &s {
        let mut temp_x = x.clone();
        for p in &primes {
            if p > &temp_x {
                break;
            }
            let mut p_count = 0;
            while temp_x % p == 0 {
                temp_x /= p;
                p_count += 1;
            }
            if p_count != 0 {
                prime_count.entry(*p).or_insert(Vec::<u32>::new()).push(p_count);
            }
        }
    }

    // test which prime factors occur N or N-1 times

    let mut ans: u64 = 1;

    for (prime, counts) in prime_count.iter_mut() {
        match counts.len() {
            x if x == s.len() => {
                counts.sort_unstable();
                ans *= (*prime as u64).pow(counts[1]);
            },
            x if x == s.len() - 1 => {
                counts.sort_unstable();
                ans *= (*prime as u64).pow(counts[0]);
            },
            _ => ()
        }
    }

    // output
    println!("{}", ans);
}

fn sieve (limit: u32) -> Vec<u32> {
    let mut primes = vec![true; limit as usize + 2];
    primes[0] = false;
    primes[1] = false;
    for i in 2..(limit as usize + 1) {
        if primes[i] == true {
            for j in ((i+i)..limit as usize).step_by(i) {
                primes[j] = false;
            }
        }
    }
    primes.iter().enumerate().filter(|(_,x)| **x).map(|(i,_)| i as u32).collect()
}
