use crate::common::get_input;
extern crate gnuplot;
extern crate nalgebra;
extern crate regex;
use gnuplot::{Caption, Color, Figure, PlotOption};
use nalgebra::{Point, Point2};
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

fn plot_wire(wire: &Vec<Point2<i32>>, figure: &mut Figure, color: PlotOption<&str>) {
    let xs: Vec<i32> = wire.into_iter().map(|p| p.x).collect();
    let ys: Vec<i32> = wire.into_iter().map(|p| p.y).collect();

    figure.axes2d().lines(&xs, &ys, &[Color("red")]);
}

fn find_intersections(wire1: &Vec<Point2<i32>>, wire2: &Vec<Point2<i32>>) -> Vec<Point2<i32>> {
    let mut intersections: Vec<Point2<i32>> = Vec::new();
    let wire1_hash: HashSet<Point2<i32>> = HashSet::from_iter(wire1.clone());
    let wire2_hash: HashSet<Point2<i32>> = HashSet::from_iter(wire2.clone());

    let hash_intersections = wire1_hash.intersection(&wire2_hash);
    for inter in hash_intersections{
        intersections.push(*inter);
    }
    return intersections;
}

fn move_wire(
    direction: Point2<i32>,
    magnitude: i32,
    current_position: &mut Point2<i32>,
    wire: &mut Vec<Point2<i32>>,
) {
    for i in 0..magnitude {
        current_position.x += direction.x;
        current_position.y += direction.y;
        wire.push(current_position.clone());
    }
}

fn steps_to_point(wire1: & Vec<Point2<i32>>,wire2: & Vec<Point2<i32>>,point:&Point2<i32>) -> usize{
    let steps = wire1.iter().position(|p| p == point).unwrap() + wire2.iter().position(|p| p == point).unwrap();
    println!("distance to {} = {}",point,steps);
    return steps;
}

pub fn star1() {
    let wire_description = get_input("inputs/day3.txt", "\n");
    let mut wires: Vec<Vec<Point2<i32>>> = Vec::new();
    let re = Regex::new(r"(?P<direction>[UDLR])(?P<magnitude>[0-9]+)").unwrap();

    let mut figure = Figure::new();
    for desc in &wire_description {
        let mut current_position: Point2<i32> = Point2::new(0, 0);
        let mut new_wire: Vec<Point2<i32>> = Vec::new();

        for segment_capture in re.captures_iter(desc) {
            let direction = segment_capture.name("direction").unwrap().as_str();
            let magnitude = segment_capture.name("magnitude").unwrap().as_str();
            println!("Segment {} {}", direction, magnitude);

            let parsed_direction: Point2<i32> = match direction {
                "U" => Point2::new(0, 1),
                "D" => Point2::new(0, -1),
                "L" => Point2::new(1, 0),
                "R" => Point2::new(-1, 0),
                _ => Point2::new(0, 0),
            };

            move_wire(
                parsed_direction,
                magnitude.parse::<i32>().unwrap(),
                &mut current_position,
                &mut new_wire,
            );
        }
        wires.push(new_wire);
    }

    plot_wire(&wires[0], &mut figure, Color("blue"));

    let intersections = find_intersections(&wires[0], &wires[1]);
    for inter in &intersections {
        println!("Intersect {},{}", inter.x, inter.y);
    }

    let distances:Vec<i32>= intersections
        .iter()
        .map(|p| p.x.abs() + p.y.abs()).collect();
    let distance =     distances.iter()
    .min()
    .unwrap();
    println!("Distance {} ", distance);

    let fewest_steps = intersections.iter().map(|intersection| steps_to_point(&wires[0], &wires[1], intersection)).min().unwrap();
    println!("Fewest steps {} ", fewest_steps);
}
