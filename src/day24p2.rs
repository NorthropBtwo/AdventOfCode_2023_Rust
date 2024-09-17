use crate::{DayFunc, DayRiddle};

//const INPUT_PATH: &str = "src/day24/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 24,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "first_try", func : first_try},
    ]
}

pub fn solution() -> u64 {
    566914635762564
}


pub fn first_try() -> u64 {
    566914635762564 //used Ocatve solver
}

//We use 3 lines:
//C=d+e*w, A=f+g*t, B=h+i*u whre w,t,u are scalar varaibles which are used for the time. A,B,C are Points we want to find. The oder symbols are constant and defined
//The goal is to find a new line which intersects ech line at its Point A,B and C. The new line is defined as P+vel*z=[A if z=t, B if z=u, C if z=2]
//The new line will go through each point in the following order C->A->B
//the new line is not a cruve, so the velocity a from C->A and A->B must be the same
//vel = (A-C)/z = (B-A)/v where z is the time it takes from C->A and v is the time it takes from A->B
//if we replace A,B,C,z and v, we get the follwoing formula:
//(f+g*t-(d+e*w))/(t-w) = (h+i*u - (f+g*t))/(u-t)
//Note that w+z=t and t+v=u was used to replace the denominator
//If the formula above is taken apart in its x,y,z componentzs, there are 3 formulas with 3 unknown. This can be solved in Octave.

