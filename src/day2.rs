
fn update(input: &str) -> i32{
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input.lines().into_iter(){
        match line.split_once(' ').unwrap(){
            ("forward",x) => horizontal += x.parse::<i32>().unwrap(),
            ("up",x) => depth -= x.parse::<i32>().unwrap(),
            ("down",x) => depth += x.parse::<i32>().unwrap(),
            _ => ()
        }
    }
    horizontal*depth
}

fn update_2(input: &str) -> i32{
    let (horizontal,depth,_) = input.lines().fold((0,0,0), |(horizontal, depth, aim) , x| {
        match x.split_once(' ').unwrap(){
            ("forward",x) => (horizontal + x.parse::<i32>().unwrap(),depth + aim * x.parse::<i32>().unwrap(),aim),
            ("up",x) => (horizontal,depth,aim - x.parse::<i32>().unwrap()),
            ("down",x) => (horizontal,depth,aim + x.parse::<i32>().unwrap()),
            _ => (horizontal,depth,aim)
        }
    });
    horizontal*depth
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day2.txt").expect("ERROR");
    println!("First submarine  -> {}",update(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day2.txt").expect("ERROR");
    println!("Second submarine -> {}",update_2(&input));
}