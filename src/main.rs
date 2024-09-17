use std::time::{Instant, Duration};

mod day1p1;
mod day1p2;
mod day2p1;
mod day2p2;
mod day3p1;
mod day3p2;
mod day4p1;
mod day4p2;
mod day5p1;
mod day5p2;
mod day6p1;
mod day6p2;
mod day7p1;
mod day7p2;
mod day8p1;
mod day8p2;
mod day9p1;
mod day9p2;
mod day10p1;
mod day10p2;
mod day11p1;
mod day11p2;
mod day12p1;
mod day12p2;
mod day13p1;
mod day13p2;
mod day14p1;
mod day14p2;
mod day15p1;
mod day15p2;
mod day16p1;
mod day16p2;
mod day17p1;
mod day17p2;
mod day18p1;
mod day18p2;
mod day19p1;
mod day19p2;
mod day20p1;
mod day20p2;
mod day21p1;
mod day21p2;
mod day22p1;
mod day22p2;
mod day23p1;
mod day23p2;
mod day24p1;
mod day24p2;
mod day25p1;

pub struct DayFunc {
    pub name : &'static str,
    pub func : fn() -> u64,
}

pub struct DayRiddle {
    pub day_nr : u32,
    pub part_nr : u32,
    pub day_funcs : Vec<DayFunc>,
    pub solution : u64,
}

const ITERATIONS : u32 = 1;

fn main() {

    //std::env::set_var("RUST_BACKTRACE", "1");

    let riddles = vec![ 
        day1p1::get_riddle(), day1p2::get_riddle(), 
        day2p1::get_riddle(), day2p2::get_riddle(),
        day3p1::get_riddle(), day3p2::get_riddle(),
        day4p1::get_riddle(), day4p2::get_riddle(),
        day5p1::get_riddle(), day5p2::get_riddle(),
        day6p1::get_riddle(), day6p2::get_riddle(),
        day7p1::get_riddle(), day7p2::get_riddle(),
        day8p1::get_riddle(), day8p2::get_riddle(),
        day9p1::get_riddle(), day9p2::get_riddle(),
        day10p1::get_riddle(), day10p2::get_riddle(),
        day11p1::get_riddle(), day11p2::get_riddle(),
        day12p1::get_riddle(), day12p2::get_riddle(),
        day13p1::get_riddle(), day13p2::get_riddle(),
        day14p1::get_riddle(), day14p2::get_riddle(),
        day15p1::get_riddle(), day15p2::get_riddle(),
        day16p1::get_riddle(), day16p2::get_riddle(),
        day17p1::get_riddle(), day17p2::get_riddle(),
        day18p1::get_riddle(), day18p2::get_riddle(),
        day19p1::get_riddle(), day19p2::get_riddle(),
        day20p1::get_riddle(), day20p2::get_riddle(),
        day21p1::get_riddle(), day21p2::get_riddle(),
        day22p1::get_riddle(), day22p2::get_riddle(),
        day23p1::get_riddle(), day23p2::get_riddle(),
        day24p1::get_riddle(), day24p2::get_riddle(),
        day25p1::get_riddle(),
        ];

    println!("{:<27}: {:<15}, {}", "solution name", "calc time", "result ok");

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
            println!("{:<27}: {:<15?}, {}", day_func.name, total_duration / ITERATIONS, result == riddle.solution);
            if result != riddle.solution {
                println!("result:{}, riddle.solution:{}", result , riddle.solution);
            }
        }
    }

}
