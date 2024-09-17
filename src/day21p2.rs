use core::str;
use std::{fs, mem::swap};

use cgmath::Vector2;
use itertools::Itertools;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day21/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 21,
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
    620962518745459
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let layout = input_string.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let mut positions_on_even_field = get_number_of_positions(&layout, 133); //we call the startungfield a even field, the Elf wants to make an uneven number of steps on this field. He wants to make a lot of steps, but they repeat one hea reaches the end of the field
    let mut positions_on_uneven_field = get_number_of_positions(&layout, 132); //every field adjacent to a even field will be an uneven field, because the elve staring poition changed

    offset_coord_system(&mut positions_on_even_field, Vector2::new(-65, -65));
    offset_coord_system(&mut positions_on_uneven_field, Vector2::new(-65, -65));

    //print_positons(&layout, &positions_on_uneven_field);

    //we change the coordinate system so that the starting point is a (0/0). (right/down)
    let mut sum = 0;
    //the most right field is reached at the coordinate (26501235/0) with 130 steps left an is a even field
    sum += get_numer_of_points_within_reach(&layout,&positions_on_even_field, Vector2::new(-65, 0) , 130);
    //on the bottom of this field is a uneven field with only (130-66) steps left
    let down_right_1 = get_numer_of_points_within_reach(&layout,&positions_on_uneven_field, Vector2::new(-65, -65) , 64);
    //this field exists 202300 times
    sum += down_right_1 * 202300;
    //on the left of the last field there is a similar field as the most right field but with more steps (64+131) left and a different start point
    let down_right_2 = get_numer_of_points_within_reach(&layout,&positions_on_even_field, Vector2::new(-65, -65) , 195);
    //this field exists 202299 times
    sum += down_right_2 * 202299;

    //the bottom most field
    sum += get_numer_of_points_within_reach(&layout,&positions_on_even_field, Vector2::new(0, -65) , 130);
    //on the left of this field is a uneven field with only (130-66) steps left
    let down_left_1 = get_numer_of_points_within_reach(&layout,&positions_on_uneven_field, Vector2::new(65, -65) , 64);
    //this field exists 202300 times
    sum += down_left_1 * 202300;
    //on the left of the last field there is a similar field as the most right field but with more steps (64+131) left and a different start point
    let down_left_2 = get_numer_of_points_within_reach(&layout,&positions_on_even_field, Vector2::new(65, -65) , 195);
    //this field exists 202299 times
    sum += down_left_2 * 202299;

    //the left most field
    sum += get_numer_of_points_within_reach(&layout,&positions_on_even_field, Vector2::new(65, 0) , 130);
    //on the left of this field is a uneven field with only (130-66) steps left
    let up_left_1 = get_numer_of_points_within_reach(&layout,&positions_on_uneven_field, Vector2::new(65, 65) , 64);
    //this field exists 202300 times
    sum += up_left_1 * 202300;
    //on the left of the last field there is a similar field as the most right field but with more steps (64+131) left and a different start point
    let up_left_2 = get_numer_of_points_within_reach(&layout,&positions_on_even_field, Vector2::new(65, 65) , 195);
    //this field exists 202299 times
    sum += up_left_2 * 202299;

    //the up most field
    sum += get_numer_of_points_within_reach(&layout,&positions_on_even_field, Vector2::new(0, 65) , 130);
    //on the left of this field is a uneven field with only (130-66) steps left
    let up_right_1 = get_numer_of_points_within_reach(&layout,&positions_on_uneven_field, Vector2::new(-65, 65) , 64);
    //this field exists 202300 times
    sum += up_right_1 * 202300;
    //on the left of the last field there is a similar field as the most right field but with more steps (64+131) left and a different start point
    let up_right_2 = get_numer_of_points_within_reach(&layout,&positions_on_even_field, Vector2::new(-65, 65) , 195);
    //this field exists 202299 times
    sum += up_right_2 * 202299;

    //now we calculate the full fields
    //there are 404599 full fields with wit x=0. 202299 even and 202300 uneven fields
    let mut even_fields = 202299;
    let mut uneven_fields= 202300;
    
    let mut sum_even = even_fields;
    let mut sum_uneven = uneven_fields;
    while even_fields >= 2 || uneven_fields >= 2 {
        swap(&mut even_fields, &mut uneven_fields); //going one to left swpas even with uneven fields
        even_fields -= 2; //we lose top and bottom must field

        sum_even += even_fields * 2; //right and left
        sum_uneven += uneven_fields * 2; //right and left
    }


    sum += (sum_even * positions_on_even_field.len() + sum_uneven * positions_on_uneven_field.len()) as u64;

    sum
}


fn get_numer_of_points_within_reach(_layout: &Vec<Vec<u8>>,points: &Vec<Vector2<isize>>,start_point: Vector2<isize>, steps_left: isize) -> u64 {
    let mut counter = 0;
    
    let mut print_points: Vec<Vector2<isize>> = vec![];

    for point in points {
        if ((point.x - start_point.x).abs() + (point.y - start_point.y).abs()) <= steps_left {
            counter += 1;
            print_points.push(*point);
        }
    }

    //print_positons(&layout, &print_points);

    counter
}


fn get_number_of_positions(layout: &Vec<Vec<u8>>, num_of_steps: usize) -> Vec<Vector2<isize>> {

    let y_max = layout.len() as isize;
    let x_max = layout[0].len() as isize;
    let mut positions = vec![find_start(&layout)];

    for _ in 0..num_of_steps {
        let mut new_positions = vec![];
        for position in positions {
            
            let next_steps = [position + Vector2::unit_x(), position - Vector2::unit_x(), position + Vector2::unit_y(), position - Vector2::unit_y()];
            for next_step in next_steps {
                if next_step.x >= 0 && next_step.x < x_max && next_step.y >= 0 && next_step.y < y_max {
                    let tile = layout[next_step.y as usize][next_step.x as usize];
                    if (tile == b'.' || tile == b'S') && !new_positions.contains(&next_step) {
                        new_positions.push(next_step)
                    }
                }
            }

        }

        positions = new_positions;
    }
    positions
}


fn find_start(layout : &Vec<Vec<u8>>)-> Vector2<isize> {
    for y in 1..layout.len() {
        for x in 1..layout[0].len() {
            if layout[y][x] == b'S' {
                return Vector2::new(x as isize, y as isize);
            }
        }
    }

    panic!("start not found");
}

fn offset_coord_system(points: &mut Vec<Vector2<isize>>, offset: Vector2<isize>) {
    for point in points {
        *point += offset;
    }
}

#[allow(dead_code)]
fn print_positons(layout : &Vec<Vec<u8>>, points: &Vec<Vector2<isize>>) {
    let mut layout = layout.clone();

    for point in points {
        layout[(point.y + 65) as usize][(point.x + 65) as usize] = b'O';
    }

    let a = layout.iter().map(|x| String::from_utf8(x.to_vec()).unwrap()).join("\r\n");
    println!("{}\r\n\r\n\r\n", a);
}

