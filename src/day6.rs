
fn lantern_age_count(input: &String, cycles: i32) -> usize{
    let ages = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut counter:[usize;9] = [0;9];
    for fish in ages {
        counter[fish] += 1;
        }
    for _ in 0..cycles{
        let new_fish = counter[0];
        counter[0] = counter[1];
        counter[1] = counter[2];
        counter[2] = counter[3];
        counter[3] = counter[4];
        counter[4] = counter[5];
        counter[5] = counter[6];
        counter[6] = counter[7] + new_fish;
        counter[7] = counter[8];
        counter[8] = new_fish;
    }
    counter.into_iter().fold(0 ,|acc , x| acc + x)
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    println!("Lanternfish After 80 days  -> {}",lantern_age_count(&input,80));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    println!("Lanternfish After 256 days -> {}",lantern_age_count(&input,256));
}