use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day7() {
    println!("== Day 7 ==");
    let input = "src/day7/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

fn part_a(input: &str) -> usize {
    let open = File::open(input).unwrap();
    let mut hands = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand = hand.split("")
            .filter(|s| !s.is_empty())
            .map(|s| match s {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                "T" => 10,
                _ => s.parse::<usize>().unwrap()
            })
            .collect::<Vec<usize>>();
        hands.push((hand, bid.parse::<usize>().unwrap()))
    }
    hands.sort_by(|(a, _), (b, _)|
        hand_score_2(a).cmp(&hand_score_2(b))
            .then(a[0].cmp(&b[0]))
            .then(a[1].cmp(&b[1]))
            .then(a[2].cmp(&b[2]))
            .then(a[3].cmp(&b[3]))
            .then(a[4].cmp(&b[4]))
    );
    // println!("{:?}", hands);
    let mut sum = 0;
    for i in 0..hands.len() {
        sum += hands.get(i)
            .map(|(_, bid)| *bid)
            .map(|bid| bid * (i + 1))
            .unwrap();
    }
    sum
}

fn _hand_score(hand: &Vec<usize>) -> usize {
    // == Best to worst ==
    // 5-of-a-kind  AAAAA == 7, 1 key
    // 4-of-a-kind  AAAAK == 6, 2 keys
    // Full house   AAAKK == 5, 2 keys
    // 3-of-a-kind  AAAKQ == 4, 3 keys, 3 1 1
    // Two pairs    AAKKQ == 3, 3 keys, 2 2 1
    // One pair     AAKQJ == 2, 4 keys
    // High card    A2345 == 1, 5 keys
    let mut map: HashMap<usize, usize> = HashMap::new();
    for card in hand.iter() {
        *map.entry(*card).or_default() += 1;
    }
    match map.keys().len() {
        1 => 7,
        2 => match map.get(&hand[0]).unwrap() {
            1 => 6,
            4 => 6,
            2 => 5,
            3 => 5,
            _ => panic!("Can't figure out what 2 is correct")
        }
        3 => match map.values().collect::<Vec<&usize>>().contains(&&3) {
            true => 4,
            false => 3,
        },
        4 => 2,
        5 => 1,
        _ => panic!("Unreasonable amount of keys: {:?}", map)
    }
}

fn hand_score_2(hand: &Vec<usize>) -> usize {
    let set: HashSet<usize> = HashSet::from_iter(hand.clone().into_iter());

    let mut counts = set.iter()
        .map(|i| hand.iter().filter(|h| *h == i).count())
        .collect::<Vec<usize>>();
    counts.sort();

    match counts.as_slice() {
        [5] => 7,
        [1, 4] => 6,
        [2, 3] => 5,
        [1, 1, 3] => 4,
        [1, 2, 2] => 3,
        [1, 1, 1, 2] => 2,
        _ => 1,
    }
}


fn part_b(input: &str) -> usize {
    let open = File::open(input).unwrap();
    let mut hands = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand = hand.split("")
            .filter(|s| !s.is_empty())
            .map(|s| match s {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 1,
                "T" => 10,
                _ => s.parse::<usize>().unwrap()
            })
            .collect::<Vec<usize>>();
        hands.push((hand, bid.parse::<usize>().unwrap()))
    }
    hands.sort_by(|(a, _), (b, _)|
        hand_score_b2(a).cmp(&hand_score_b2(b))
            .then(a[0].cmp(&b[0]))
            .then(a[1].cmp(&b[1]))
            .then(a[2].cmp(&b[2]))
            .then(a[3].cmp(&b[3]))
            .then(a[4].cmp(&b[4]))
    );
    // println!("{:?}", hands);
    let mut sum = 0;
    for i in 0..hands.len() {
        sum += hands.get(i)
            .map(|(_, bid)| *bid)
            .map(|bid| bid * (i + 1))
            .unwrap();
    }
    sum
}

fn _hand_score_b(hand: &Vec<usize>) -> usize {
    // == Best to worst ==
    // 5-of-a-kind  AAAAA == 7, 1 key
    // 4-of-a-kind  AAAAK == 6, 2 keys
    // Full house   AAAKK == 5, 2 keys
    // 3-of-a-kind  AAAKQ == 4, 3 keys, 3 1 1
    // Two pairs    AAKKQ == 3, 3 keys, 2 2 1
    // One pair     AAKQT  == 2, 4 keys
    // High card    A2345 == 1, 5 keys
    //
    // If The hand contains a J (or 1 in this case) the 1 turns into the best card for the hand
    if hand.contains(&1) {
        // Do the replace logic

        let mut map: HashMap<usize, usize> = HashMap::new(); // <card, occurrences>
        for card in hand.iter().filter(|i| **i != 1) {
            *map.entry(*card).or_default() += 1;
        }
        let mut inverted_map: HashMap<usize, Vec<usize>> = HashMap::new(); // <occurrences, Vec<card>>
        for (k, v) in map.iter() {
            inverted_map.entry(*v).or_default().push(*k);
        }

        // println!("hand {:?}", hand);
        // println!("map {:?}", map);
        // println!("inverted_map {:?}", inverted_map);
        let new_hand = match map.keys().len() {
            1 => hand.iter() // AAAAJ
                .map(|c| match c {
                    &1 => map.keys().last().unwrap(),
                    &_ => c
                })
                .map(|i| *i)
                .collect(),
            2 => hand.iter() // AAAKJ || AAKKJ || AKJJJ
                .map(|c| {
                    match c {
                        // &1 => inverted_map.get(&3)
                        //     .map(|v| v.get(0).unwrap())
                        //     .unwrap_or( inverted_map.get(&2)
                        //         .map(|v| v.iter().max().unwrap())
                        //         .unwrap()
                        //     ),
                        &1 => {
                            if inverted_map.get(&3).is_some() {
                                inverted_map.get(&3).unwrap().get(0).unwrap()
                            } else if inverted_map.get(&2).is_some() {
                                inverted_map.get(&2).unwrap().iter().max().unwrap()
                            } else {
                                inverted_map.get(&1).unwrap().iter().max().unwrap()
                            }
                        }
                        &_ => c
                    }
                })
                .map(|i| *i)
                .collect(),
            3 => hand.iter() // AAKQJ || AKQJJ
                .map(|c| match c {
                    // &1 => inverted_map.get(&2).unwrap().get(0).unwrap(),
                    &1 => {
                        if inverted_map.get(&2).is_some() {
                            inverted_map.get(&2).unwrap().get(0).unwrap()
                        } else {
                            inverted_map.get(&1).unwrap().iter().max().unwrap()
                        }
                    }
                    &_ => c
                })
                .map(|i| *i)
                .collect(),
            4 => hand.iter() // AKQTJ
                .map(|c| match c {
                    &1 => hand.iter().max().unwrap(),
                    &_ => c
                })
                .map(|i| *i)
                .collect(),
            _ => hand.clone()
        };//.iter().map(|i| *i).collect();

        // println!("Old hand: {:?}\nNew hand: {:?}", hand, new_hand);
        return _hand_score(&new_hand);
    }
    // If there is no Joker, rank as normal
    _hand_score(hand)
}

fn hand_score_b2(hand: &Vec<usize>) -> usize {
    let mut set: HashSet<usize> = HashSet::from_iter(hand.clone().into_iter());
    set.remove(&1);

    let mut counts = set.iter()
        .map(|i| hand.iter().filter(|h| *h == i).count())
        .collect::<Vec<usize>>();
    counts.sort();

    let missing = 5 - counts.iter().sum::<usize>();

    match counts.last_mut() {
        None => counts.push(5),
        Some(v) => *v += missing,
    }

    match counts.as_slice() {
        [5] => 7,
        [1, 4] => 6,
        [2, 3] => 5,
        [1, 1, 3] => 4,
        [1, 2, 2] => 3,
        [1, 1, 1, 2] => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day7();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day7/input.txt";
        assert_eq!(246795406, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day7/input.txt";
        assert_eq!(249356515, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day7/test-input.txt";
        let result = part_a(input);
        assert_eq!(6440, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day7/test-input.txt";
        let result = part_b(input);
        assert_eq!(5905, result);
    }
}