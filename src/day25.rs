use std::time::Instant;


fn step_left(field: &Vec<Vec<char>>) -> (Vec<Vec<char>>,bool){
    let mut new = field.clone();
    let mut check = true;
    for row in 0..field[0].len()-1{
        for col in 0..field.len(){
            if field[col][row] == '>' && field[col][row + 1] == '.'{
                check = false;
                new[col][row] = '.';
                new[col][row + 1] = '>';
            }
        }
    }
    let row = field[0].len()-1;
    for col in 0..field.len(){
        if field[col][row] == '>' && field[col][0] == '.'{
            check = false;
            new[col][row] = '.';
            new[col][0] = '>';
        }
    }
    (new,check)
}

fn step_down(field: &Vec<Vec<char>>) -> (Vec<Vec<char>>,bool){
    let mut new = field.clone();
    let mut check = true;
    for col in 0..field.len()-1{
        for row in 0..field[0].len(){
            if field[col][row] == 'v' && field[col + 1][row] == '.'{
                check = false;
                new[col][row] = '.';
                new[col + 1][row] = 'v';
            }
        }
    }
    let col = field.len()-1;
    for row in 0..field[0].len(){
        if field[col][row] == 'v' && field[0][row] == '.'{
            check = false;
            new[col][row] = '.';
            new[0][row] = 'v';
        }
    }
    (new,check)
}

fn landing(input: &str) -> i32{
    let mut field = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut left_check: bool;
    let mut down_check: bool;
    let mut res = 0;
    loop{
        res += 1;
        (field,left_check) = step_left(&mut field);
        (field,down_check) = step_down(&mut field);
        if left_check && down_check {break;}
    }
    res
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day25.txt").unwrap();
    let now = Instant::now();
    println!("Steps to land -> {} in {} ms",landing(&input),now.elapsed().as_millis());
}

pub fn part2(){

}