use std::{fs, str::FromStr, collections::HashMap};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day7/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 7,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "First Try", func : first_try},
        DayFunc{name: "with_sorded_vec", func : with_sorded_vec},
        DayFunc{name: "with_precalc_strength", func : with_precalc_strength},
        DayFunc{name: "strength_no_structs", func : strength_no_structs},
        DayFunc{name: "strength_no_hash_map", func : strength_no_hash_map},
        DayFunc{name: "timvisee(not mine)", func : timvisee},
    ]
}

pub fn solution() -> u64 {
    248105065
}

#[derive(Debug, Eq)]
struct Player {
    hand: [Cards; 5],
    bid_amount : u32,
    hand_type: HandType,
    strength: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePlayerError;

impl FromStr for Player {
    type Err = ParsePlayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let mut word_iter = s.split_ascii_whitespace();

        let mut hand = word_iter.next().unwrap_or_default().chars();
        let bid_amount = word_iter.next().unwrap_or_default().parse::<u32>().map_err(|_|ParsePlayerError)?;

        let hand = [hand.next().ok_or(ParsePlayerError)?,hand.next().ok_or(ParsePlayerError)?,hand.next().ok_or(ParsePlayerError)?,hand.next().ok_or(ParsePlayerError)?,hand.next().ok_or(ParsePlayerError)?];
        let hand = Cards::from_char_array(hand).map_err(|_|ParsePlayerError)?;
        
        let mut m: HashMap<Cards, u32> = HashMap::new();
        for c in &hand {
            *m.entry(*c).or_default() += 1;
        }

        let mut a : Vec<u32> = m.into_values().collect();
        a.sort();
        let hand_type = match (a[a.len()-1], a.get(a.len().wrapping_sub(2))) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, Some(2)) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, Some(2)) => HandType::TwoPair,
            (2, Some(1)) => HandType::OnePair,
            _ => HandType::HighCard,
        };

        let mut strength = hand_type as u64;
        for card in hand {
            strength = (strength << 4) + (card as u64);
        }

        Ok(Player { hand, bid_amount, hand_type, strength })
    }
}


impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => self.hand.cmp(&other.hand),
        }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}




#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Cards {
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    T,
    J,
    Q,
    K,
    A,
}

impl Cards {
    pub fn from_char(c: char) -> Result<Cards, ()> {
        match c {
            '2' => Ok(Cards::D2),
            '3' => Ok(Cards::D3),
            '4' => Ok(Cards::D4),
            '5' => Ok(Cards::D5),
            '6' => Ok(Cards::D6),
            '7' => Ok(Cards::D7),
            '8' => Ok(Cards::D8),
            '9' => Ok(Cards::D9),
            'T' => Ok(Cards::T),
            'J' => Ok(Cards::J),
            'Q' => Ok(Cards::Q),
            'K' => Ok(Cards::K),
            'A' => Ok(Cards::A),
            _ => Err(()),
        }
    }

    pub fn from_char_array(c: [char; 5]) -> Result<[Cards; 5], ()> {
        Ok([Cards::from_char(c[0])?,Cards::from_char(c[1])?,Cards::from_char(c[2])?,Cards::from_char(c[3])?,Cards::from_char(c[4])?])
    }
}


#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut players = vec![];
    let mut rank = 1;
    let mut sum = 0;


    for line in input_string.lines() {
        if let Ok(player) = Player::from_str(line) {
            players.push(player);
        }
    }

    players.sort_unstable();

    for player in players {
        sum += player.bid_amount * rank;
        rank += 1;
    }

    sum as u64
}

pub fn with_sorded_vec() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut players = sorted_vec::SortedVec::new();
    let mut rank = 1;
    let mut sum = 0;


    for line in input_string.lines() {
        if let Ok(player) = Player::from_str(line) {
            players.push(player);
        }
    }

    for player in players.iter() {
        sum += player.bid_amount * rank;
        rank += 1;
    }

    sum as u64
}

pub fn with_precalc_strength() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut players = vec![];
    let mut rank = 1;
    let mut sum = 0;


    for line in input_string.lines() {
        if let Ok(player) = Player::from_str(line) {
            players.push(player);
        }
    }

    players.sort_by(|a, b| a.strength.cmp(&b.strength));

    for player in players {
        sum += player.bid_amount * rank;
        rank += 1;
    }

    sum as u64
}

pub fn strength_no_structs() -> u64 {
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
                'T' => b'0' + 10,
                'J' => b'0' + 11,
                'Q' => b'0' + 12,
                'K' => b'0' + 13,
                'A' => b'0' + 14,
                n => n as u8,
            }
        );

        let mut m: HashMap<u8, u32> = HashMap::with_capacity(13);
        for c in &hand {
            *m.entry(*c).or_default() += 1;
        }
        let mut a : Vec<u32> = m.into_values().collect();
        a.sort_unstable();
        let strength_bucket_idx: usize = match (a[a.len()-1], a.get(a.len().wrapping_sub(2))) {
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


pub fn strength_no_hash_map() -> u64 {
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
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                n => (n as u8) -b'2',
            }
        );

        let mut a = [0; 13];
        for card in &hand {
            a[(*card) as usize] += 1;
        }
        a.sort_unstable();
        let strength_bucket_idx: usize = match (a[a.len()-1], a.get(a.len().wrapping_sub(2))) {
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
        let (mut ranks, mut power) = ([0u8; 13], 0);
        for i in 0..5 {
            let card = match hand[i] {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' => 9,
                b'T' => 8,
                n => n - b'0' - 2,
            };
            ranks[card as usize] += 1;
            power |= (card as u32) << 4 * (4 - i);
        }
        ranks.sort_unstable_by(|a, b| b.cmp(a));
        power |= match ranks[0] {
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


    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_power, bet))| bet * (i as u32 + 1))
        .sum::<u32>() as u64
}