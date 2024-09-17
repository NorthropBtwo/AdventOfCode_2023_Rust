use std::{fs, mem};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day7/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 7,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "from_part_1", func : from_part_1},
        DayFunc{name: "timvisee(not mine)", func : timvisee},
    ]
}

pub fn solution() -> u64 {
    249515436
}


pub fn from_part_1() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut rank = 1;
    let mut sum = 0;
    const ARRAY_REPEAT_VALUE: Vec<(u64, u32)> = Vec::new();
    let mut strength_buckets: [Vec<(u64, u32)>; 7] = [ARRAY_REPEAT_VALUE; 7];


    for line in input_string.lines() {

        let mut word_iter = line.split_ascii_whitespace();
        let mut hand = word_iter.next().unwrap_or_default().chars();
        let bid_amount = word_iter.next().unwrap_or_default().parse::<u32>().unwrap();
        let hand = [hand.next().unwrap(),hand.next().unwrap(),hand.next().unwrap(),hand.next().unwrap(),hand.next().unwrap()];
        let hand = hand.map(|h| 
            match h {
                'T' => 9,
                'J' => 0,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                n => (n as u8) -b'1',
            }
        );

        let mut a = [0; 13];
        for card in &hand {
            a[(*card) as usize] += 1;
        }
        let mut num_of_jokers = 0;
        mem::swap(&mut num_of_jokers, &mut a[0]);
        a.sort_unstable();
        let strength_bucket_idx: usize = match (a[a.len()-1] + num_of_jokers, a.get(a.len().wrapping_sub(2))) {
            (5, _) => 6,
            (4, _) => 5,
            (3, Some(2)) => 4,
            (3, _) => 3,
            (2, Some(2)) => 2,
            (2, Some(1)) => 1,
            _ => 0,
        };

        let mut strength = 0;
        for card in hand {
            strength = (strength << 4) + (card as u64);
        }

        strength_buckets[strength_bucket_idx].push((strength , bid_amount));
    }

    for i in 0..=6 {
        strength_buckets[i].sort_unstable_by(|a, b| a.0.cmp(&b.0))
    }


    for player in strength_buckets.iter().flatten() {
        sum += player.1 * rank; /*player.bid_amount * rank;*/
        rank += 1;
    }

    sum as u64
}



fn timvisee() -> u64 {

    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let mut hands = input_string.as_bytes()
    .split(|b| b == &b'\n')
    .map(|hand| {
        let (mut ranks, mut power, mut jokers) = ([0u8; 13], 0, 0);
        for i in 0..5 {
            let card = match hand[i] {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' => 0,
                b'T' => 9,
                n => n - b'0' - 1,
            };
            ranks[card as usize] += 1 * (card != 0) as u8;
            power |= (card as u32) << 4 * (4 - i);
            jokers += 1 * (card == 0) as u8;
        }
        ranks.sort_unstable_by(|a, b| b.cmp(a));
        power |= match ranks[0] + jokers {
            5 => 6,
            4 => 5,
            3 if ranks[1] == 2 => 4,
            3 => 3,
            2 if ranks[1] == 2 => 2,
            2 => 1,
            _ => 0,
        } << 29;
        (power, atoi::atoi::<u32>(&hand[6..]).unwrap())
    })
    .collect::<Vec<_>>();
    hands.sort_unstable();

    let sum = hands
    .into_iter()
    .enumerate()
    .map(|(i, (_power, bet))| bet * (i as u32 + 1))
    .sum::<u32>();

    sum as u64

}