use std::io;
use std::collections::HashMap;

fn main() {
    let primes = sieve(200000);

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let s: Vec<u64> = input.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();

    // gcd(lcm(a,b), lcm(a,c)) = lcm(a, gcd(b,c)) 
    let mut gcds = Vec::<u64>::new();
    for (i,a) in s.iter().enumerate() {
        gcds.push(lcm(a, &gcd(s.iter().skip(i+1).map(|x| *x).collect::<Vec<u64>>(), &primes), &primes));
    }

    println!("{}", gcd(gcds, &primes).unwrap());
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

fn gcd(s: Vec<u64>, primes: &Vec<u32>) -> Option<u64> {
    if s.len() == 0 { return None; }

    let mut prime_count = HashMap::<u32, Vec<u32>>::new();

    for x in &s {
        let mut temp_x = x.clone() as u32;
        for p in primes {
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

    let mut ans = 1;
    for (prime, count) in prime_count.iter_mut() {
        if count.len() == s.len() {
            count.sort_unstable();
            ans *= (prime.pow(count[0])) as u64;
        }
    }
    Some(ans)

}

fn lcm(a: &u64, b: &Option<u64>, primes: &Vec<u32>) -> u64 {
    if let Some(x) = b {
        a * x / gcd(vec![*a,*x], primes).unwrap()
    } else {
        *a
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gcd() {
        let primes = super::sieve(200000);
        assert_eq!(super::gcd(vec![8, 12, 20], &primes), Some(4));
    }

    #[test]
    fn test_gcd_empty() {
        let primes = super::sieve(200000);
        assert_eq!(super::gcd(vec![], &primes), None);
    }

    #[test]
    fn test_lcm() {
        let primes = super::sieve(200000);
        assert_eq!(super::lcm(&12, &Some(15), &primes), 60)
    }
}
