use crate::common::get_input;

pub fn calculate_fuel(mass: i32) -> i32 {
    if mass > 0 {
        let result: i32 = (mass / 3) - 2;
        if result > 0 {
            return result;
        }
    }
    0
}

pub fn calculate_fuel_with_fuel(mass: i32) -> i32 {
    if mass > 0 {
        let mut fuel_for_module: i32 = calculate_fuel(mass);
        if fuel_for_module > 0 {
            let mut fuel_for_fuel: i32 = calculate_fuel(fuel_for_module);
            println!("Extra fuel: {}", fuel_for_fuel);
            while fuel_for_fuel > 0 {
                fuel_for_module += fuel_for_fuel;
                fuel_for_fuel = calculate_fuel(fuel_for_fuel);
                println!("Extra fuel: {}", fuel_for_fuel);
            }
            return fuel_for_module;
        }
    }
    0
}

pub fn star1() {
    let input = get_input("inputs/day1.txt", "\n");
    let result = input
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .fold(0, |total_fuel: i32, mass: i32| {
            total_fuel + calculate_fuel(mass)
        });
    println!("Day 1 Star 1: {}", result);
}

pub fn star2() {
    let input = get_input("inputs/day1.txt", "\n");
    let mut total_fuel = input
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .fold(0, |total_fuel: i32, mass: i32| {
            total_fuel + calculate_fuel_with_fuel(mass)
        });

    println!("Day 1 Star 2: {}", total_fuel);
}
