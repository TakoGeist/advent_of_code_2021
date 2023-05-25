use std::ops::RangeInclusive;

fn get_range(input: &str) -> (RangeInclusive<i32>,RangeInclusive<i32>){
    let input = input.replace("target area: ", "");
    let input = input.split_once(",").unwrap();
    let x = input.0.trim().replace("x=", "");
    let x = x.split_once("..").unwrap();
    let x_range = x.0.parse::<i32>().unwrap()..=x.1.parse::<i32>().unwrap();
    let y = input.1.trim().replace("y=", "");
    let y = y.split_once("..").unwrap();
    let y_range = y.0.parse::<i32>().unwrap()..=y.1.parse::<i32>().unwrap();
    (x_range,y_range)
}

fn max_height(input: &str) -> i32{
    let (_,y_range) = get_range(input);
    let v_y = y_range.start();
    (v_y * (v_y + 1)) / 2
}

fn velocity_count(input: &str) -> i32{
    let mut res = 0;
    let (x_range,y_range) = get_range(input);
    let min_x = (-1_f32 + f32::sqrt(8_f32 * ((*x_range.start()) as f32)) / 2_f32).ceil() as i32;
    for vel_x in min_x..=*x_range.end(){
        for vel_y in *y_range.start()..=y_range.start().abs(){
            let mut pos_x = 0;
            let mut pos_y = 0;
            let mut v_x = vel_x;
            let mut v_y = vel_y;
            while (pos_x < *x_range.end() && pos_y > *y_range.start()) 
                  && !(x_range.contains(&pos_x) && y_range.contains(&pos_y)) {
                if v_x != 0{
                    pos_x += v_x;
                    v_x -= 1;
                }
                pos_y += v_y;
                v_y -= 1;
            }
            if x_range.contains(&pos_x) && y_range.contains(&pos_y) {res += 1;}
        }
    }
    res
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day17.txt").unwrap();
    println!("Max Height achievable -> {}",max_height(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day17.txt").unwrap();
    println!("Count of Distinct Inicial Velocity -> {}",velocity_count(&input));
}