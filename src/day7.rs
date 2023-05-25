
fn crabs_best_position(input: &String) -> i32{
    let positions = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut fuel_consumption:Vec<(i32,i32)> = 
        (0..*positions.iter().max().unwrap())
        .map(|pos| (positions.iter()
        .fold(0, |acc , x| acc + (i32::abs(pos - x) * (i32::abs(pos - x) + 1) / 2)),pos))
        .collect::<Vec<(i32,i32)>>();
    fuel_consumption.sort();
    fuel_consumption[0].0
}

fn best_position(input: &String) -> i32{
    let positions = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut fuel_consumption:Vec<(i32,i32)> = 
    (0..*positions.iter().max().unwrap())
        .map(|pos| (positions.iter().fold(0, |acc , x| acc + i32::abs(pos - x)),pos))
        .collect::<Vec<(i32,i32)>>();
    fuel_consumption.sort();
    fuel_consumption[0].0
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    println!("Proposed best position -> {:?}",best_position(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    println!("Crabs best position    -> {:?}",crabs_best_position(&input));
}