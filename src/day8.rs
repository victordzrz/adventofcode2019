extern crate itertools;
use crate::common::get_input;
use itertools::Itertools;

const LAYER_LENGTH: usize = 25;
const LAYER_WIDTH: usize = 6;
const LAYER_SIZE: usize = LAYER_LENGTH * LAYER_WIDTH;

pub fn star1() {
    let input = get_input("inputs/day8.txt", "\n");
    let input_string = &input[0];

    let min_layer = input_string
        .chars()
        .chunks(LAYER_SIZE)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|digit| digit.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .map(|usize_vecs| {
            usize_vecs
                .iter()
                .fold([0u32, 0u32, 0u32, 0u32], |mut counts, digit| {
                    counts[*digit] += 1;
                    counts
                })
        })
        .min_by_key(|counts| counts[0])
        .unwrap();

    let result = min_layer[1] * min_layer[2];
    println!("Min layer : {:?}", min_layer);
    println!("Day 8 Star 1: {}", result);
}

fn stack_digits(digit_front: char, digit_back: char) -> char {
    match digit_front {
        '2' => digit_back,
        _ => digit_front,
    }
}

pub fn star2() {
    let input = get_input("inputs/day8.txt", "\n");
    let input_string = &input[0];

    let mut stacked_layer = ['2'; LAYER_SIZE];

    println!("Day 8 Star 2");

    input_string
        .chars()
        .chunks(LAYER_SIZE)
        .into_iter()
        .fold(&mut stacked_layer, |stacked_layer, layer| {
            layer.enumerate().for_each(|index_digit| {
                stacked_layer[index_digit.0] =
                    stack_digits(stacked_layer[index_digit.0], index_digit.1)
            });
            stacked_layer
        })
        .iter()
        .map(|digit| if *digit == '1' { '$' } else { ' ' })
        .chunks(LAYER_LENGTH)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .inspect(|row| println!("{}", row))
        .collect_vec();
}
