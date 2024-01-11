use std::time::{Instant, Duration};

mod day1p1;
mod day1p2;
mod day2p1;
mod day2p2;
mod day3p1;
mod day3p2;

pub struct DayFunc {
    pub name : &'static str,
    pub func : fn() -> u32,
}

pub struct DayRiddle {
    pub day_nr : u32,
    pub part_nr : u32,
    pub day_funcs : Vec<DayFunc>,
    pub solution : u32,
}

const ITERATIONS : u32 = 1000;

fn main() {

    let riddles = vec![ 
        day1p1::get_riddle(), day1p2::get_riddle(), 
        day2p1::get_riddle(), day2p2::get_riddle(),
        day3p1::get_riddle(), day3p2::get_riddle(),
        ];

    for riddle in riddles {
        println!("Day{}-part{}{}", riddle.day_nr, riddle.part_nr, "-".to_string().repeat(40));
        for day_func in riddle.day_funcs {
            let mut total_duration = Duration::ZERO;
            let mut result = 0;
            for _ in 0..ITERATIONS {
                let start = Instant::now();
                result  = (day_func.func)();
                let duration = start.elapsed();
                total_duration += duration;
            }
            println!("{:<25}: {:<15?}, {}", day_func.name, total_duration / ITERATIONS, result == riddle.solution);
        }
    }

}
