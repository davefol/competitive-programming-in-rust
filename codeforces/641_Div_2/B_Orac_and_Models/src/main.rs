use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    for _ in 0..(t.trim().parse::<u32>().unwrap()) {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let mut models = s.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        models.insert(0, 0);

        let mut memo: Vec<Option<u32>> = vec![None; models.len()];
        memo[0] = Some(0);
        memo[1] = Some(1);

        for i in 2..models.len() {
            max_beautiful(i, &models, &mut memo);
        }

        println!("{}", memo.iter().map(|x| x.unwrap()).max().unwrap());
    }
}

fn divisors(n: u32) -> Vec<u32> {
    let mut ret = Vec::<u32>::new();
    for i in 1..=((n as f32).sqrt()) as u32 {
        if n % i == 0 {
            ret.push(i);
            if n / i != i {
                ret.push(n/i);
            }
        }
    }
    ret
}

fn max_beautiful(idx: usize, models: &Vec<u32>, memo: &mut Vec<Option<u32>>) -> u32 {
    match memo[idx] {
        Some(x) => x,
        None => {
            match divisors(idx as u32).iter()
                .map(|i| *i as usize)
                .filter(|i| models[*i] < models[idx])
                .map(|i| max_beautiful(i, models, memo) + 1)
                .max() {
                Some(y) => {
                    memo[idx] = Some(y);
                    y
                },
                None => {
                    memo[idx] = Some(1);
                    1
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_divisors_of_2() {
        let ans = vec![1,2];
        let mut divs = super::divisors(2);
        divs.sort_unstable();
        assert_eq!(divs, ans);
    }
}
