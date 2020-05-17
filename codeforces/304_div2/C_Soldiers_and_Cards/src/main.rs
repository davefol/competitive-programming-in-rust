use std::io;
fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<u32>().unwrap();
    let max_iter = factorial(n+1);
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut s1: Vec<u32> = buffer.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut s1_cards = s1.split_off(1);
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut s2: Vec<u32> = buffer.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut s2_cards = s2.split_off(1);

    let mut counter = 0;
    let mut infinite_game = false;
    while s1_cards.len() > 0 && s2_cards.len() > 0 {
        
        if counter > max_iter {
            infinite_game = true;
            break;
        }

        match (s1_cards.remove(0), s2_cards.remove(0)) {
            (x, y) if x > y => {
                s1_cards.push(y);
                s1_cards.push(x);
            },
            (x, y) if x < y => {
                s2_cards.push(x);
                s2_cards.push(y);
            },
            _ => unreachable!()
        }
        counter += 1;
    }

    if infinite_game {
        println!("{}", -1);
    } else {
        let winner: u32;
        if s1_cards.len() == 0 {
            winner = 2;
        } else {
            winner = 1;
        }
        println!("{} {}", counter, winner);
    }

}

fn factorial(x: u32) -> u32 {
    (2..x).fold(1, |a, b| {
        a * b
    })
}
