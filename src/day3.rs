
fn count_one_npos(list: &Vec<Vec<i32>>, pos: usize) -> i32{
    let mut res = 0;
    for num in list{
        res += num[pos];
    }
    res
}

fn remove(list: &mut Vec<Vec<i32>>, pos: usize, value: i32){
    let mut removed = 0;
    for ind in 0..list.len(){
        if list[ind-removed][pos] == value{
            list.remove(ind - removed);
            removed += 1;
        }
    }
}

fn list_to_int(list: &Vec<i32>) -> i32{
    let mut res = 0;
    for num in list{
        res *= 2;
        res += *num;
    }
    res
}

fn power_consumption(list: &Vec<Vec<i32>>) -> i32{
    let length = list.len() as i32;
    let mut gamma = 0;
    let len = list[0].len();
    for pos in 0..len{
        gamma *= 2;
        if 2*count_one_npos(&list, pos) > length{
            gamma += 1;
        }
    }
    let epsilon = !gamma & ((1<<len)-1);
    gamma * epsilon
}

fn oxygen_rating(list: &mut Vec<Vec<i32>>) -> i32{
    let mut pos = 0;
    while list.len()>1 && pos<list[0].len(){
        if 2*count_one_npos(&list, pos) >= list.len() as i32{
            remove(list, pos, 0);
        }
        else{
            remove(list, pos, 1);
        }
        pos += 1;
    }
    list_to_int(&list[0])
}

fn c02_rating(list: &mut Vec<Vec<i32>>) -> i32{
    let mut pos = 0;
    while list.len()>1 && pos<list[0].len(){
        if 2*count_one_npos(&list, pos) < list.len() as i32{
            remove(list, pos, 0);
        }
        else{
            remove(list, pos, 1);
        }
        pos += 1;
    }
    list_to_int(&list[0])
}

fn life_support_rating(oxygen: i32, c02: i32) -> i32{
    oxygen * c02
}

pub fn part1() {
    let report = std::fs::read_to_string("input/day3.txt").expect("ERROR");
    let list = report.lines().map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    println!("Power Consumption   -> {}", power_consumption(&list));
}

pub fn part2(){
    let report = std::fs::read_to_string("input/day3.txt").expect("ERROR");
    let list = report.lines().map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    println!("Life Support Rating -> {}",life_support_rating(oxygen_rating(&mut list.clone()), c02_rating(&mut list.clone())));
}