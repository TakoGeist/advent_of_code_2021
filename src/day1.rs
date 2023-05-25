fn count_increase(measure: &Vec<i32>, sw: usize) -> i32{
    let mut count: i32 = 0;
    for num in sw..measure.len(){
        if measure[num] > measure[num-sw]{
            count += 1;
        }
    }
    count
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day1.txt").expect("ERROR");
    let input = input.lines().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    println!("Unit Step Increases  -> {}",count_increase(&input,1));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day1.txt").expect("ERROR");
    let input = input.lines().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();    
    println!("Three Step increases -> {}",count_increase(&input,3));
}