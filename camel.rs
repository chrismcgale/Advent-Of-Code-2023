use std::cmp::Ordering;
use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn kind(hand: &String) -> i32 {
    let cards: Vec<char> = hand.chars().collect();
    let mut unique: HashMap<char, i32> = HashMap::new();

    for c in cards {
        unique
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let k = unique.keys().len();

    let mut jk = 0;
    match unique.get(&("J".chars().next().unwrap())) {
        Some(amount) => jk = *amount,
        None => {}
    };

    // 5 of a kind
    if k == 1 {
        return 7;
    }

    // Four or full house
    if k == 2 {
        for v in unique.values() {
            if *v == 4 {
                if jk >= 1 {
                    return 7;
                }
                return 6;
            }
            if *v == 3 {
                if jk > 0 {
                    return 7;
                }
                return 5;
            }
        }
    }

    // Three of a kind or two pair
    if k == 3 {
        for v in unique.values() {
            if *v == 3 {
                if jk >= 1 {
                    return 6;
                }
                return 4;
            }
            if *v == 2 {
                if jk == 2 {
                    return 6;
                }
                if jk == 1 {
                    return 5;
                }
                return 3;
            }
        }
    }

    // Pair
    if k == 4 {
        if jk >= 1 {
            return 4;
        }
        return 2;
    }

    // High card
    if k == 5 {
        if jk == 1 {
            return 2;
        }
        return 1;
    }

    return 0;
}

fn main() -> std::io::Result<()> {
    let file = File::open("6.input")?;
    let reader = BufReader::new(file);
    let array: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut bets: Vec<(String, i32)> = array
        .iter()
        .map(|hand| hand.split_once(" ").unwrap())
        .map(|(cards, amount)| (cards.to_owned(), amount.parse::<i32>().unwrap()))
        .collect();

    let card_compare: HashMap<&str, i32> = hashmap!["J" => -1, "2" => 0, "3" => 1, "4" => 2, "5" => 3, "6" => 4, "7" => 5, "8" => 6, "9" => 7, "T" => 8, "Q" => 10, "K" => 11, "A" => 12];

    bets.sort_by(|bet1, bet2| {
        let a = &(*bet1).0;
        let b = &(*bet2).0;

        println!("{} {} {} {}", a, kind(a), b, kind(b));

        if kind(a) > kind(b) {
            return Ordering::Greater;
        };

        if kind(a) < kind(b) {
            return Ordering::Less;
        };

        for i in 0..5usize {
            let first = &a[i..i + 1];
            let sec = &b[i..i + 1];
            if card_compare[first] > card_compare[sec] {
                return Ordering::Greater;
            } else if card_compare.get(sec) > card_compare.get(first) {
                return Ordering::Less;
            }
        }

        return Ordering::Greater;
    });

    println!("{:?}", bets);

    let mut sum = 0;

    for (i, (_, bet)) in bets.iter().enumerate() {
        sum += ((i as i32) + 1) * bet;
    }

    println!("{}", sum);

    Ok(())
}
