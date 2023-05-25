
fn flash(field: &mut Vec<Vec<u32>>, (row,col): (usize,usize),reset: &mut Vec<(usize,usize)>) -> i32{
    let mut res = 0;
    reset.push((row,col));
    field[row][col] = 0;
    let left = row > 0;
    let right = row < field.len()-1;
    let top = col > 0;
    let bottom = col < field[0].len()-1;
    if left   {if field[row-1][col] == 9{res += flash(field, (row-1,col), reset);}else{field[row-1][col] += 1;}}
    if right  {if field[row+1][col] == 9{res += flash(field, (row+1,col), reset);}else{field[row+1][col] += 1;}}
    if top    {if field[row][col-1] == 9{res += flash(field, (row,col-1), reset);}else{field[row][col-1] += 1;}}
    if bottom {if field[row][col+1] == 9{res += flash(field, (row,col+1), reset);}else{field[row][col+1] += 1;}}
    if left && top     {if field[row-1][col-1] == 9{res += flash(field, (row-1,col-1), reset);}else{field[row-1][col-1] += 1;}}
    if left && bottom  {if field[row-1][col+1] == 9{res += flash(field, (row-1,col+1), reset);}else{field[row-1][col+1] += 1;}}
    if right && top    {if field[row+1][col-1] == 9{res += flash(field, (row+1,col-1), reset);}else{field[row+1][col-1] += 1;}}
    if right && bottom {if field[row+1][col+1] == 9{res += flash(field, (row+1,col+1), reset);}else{field[row+1][col+1] += 1;}}
    1 + res
}

fn total_flashes(input: &str) -> i32{
    let mut res = 0;
    let mut field = input.lines().map(|x| x.chars()
                                    .map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>())
                                    .collect::<Vec<Vec<u32>>>();
    let steps = 100;
    let mut reset:Vec<(usize,usize)> = Vec::new();
    for _ in 0..steps{
        reset.clear();
        for row in 0..field.len(){
            for col in 0..field[0].len(){
                if field[row][col] == 9 {res += flash(&mut field, (row,col), &mut reset);}
                else { field[row][col] += 1;}
            }
        }
        for (row,col) in reset.iter(){
            field[*row][*col] = 0;
        }
    }
    res
}

fn all_flash(input: &str) -> i32{
    let mut res = 0;
    let mut field = input.lines().map(|x| x.chars()
                                    .map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>())
                                    .collect::<Vec<Vec<u32>>>();
    let mut reset:Vec<(usize,usize)> = Vec::new();
    while !field.iter().map(|x| x.iter().fold(true, |acc , x| acc && *x == 0))
                      .fold(true,| acc , x| acc && x){
        reset.clear();
        for row in 0..field.len(){
            for col in 0..field[0].len(){
                if field[row][col] == 9 {_ =  flash(&mut field, (row,col), &mut reset);}
                else { field[row][col] += 1;}
            }
        }
        for (row,col) in reset.iter(){
            field[*row][*col] = 0;
        }
        res += 1;
    }
    res
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("Total Flashes     -> {}",total_flashes(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("All Flash at Step -> {}",all_flash(&input));
}