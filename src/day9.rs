
fn find_low_points(map: &Vec<Vec<i32>>) -> Vec<(usize,usize)>{
    let mut low_points: Vec<(usize,usize)> = Vec::new();
    let row_len = map.len();
    let col_len = map[0].len();
    for (row,line) in map.iter().enumerate(){
        for (col,point) in line.iter().enumerate(){
            let mut low = true;
            if row > 0 { low = low && map[row-1][col] > *point;}
            if row < row_len-1 { low = low && map[row+1][col] > *point;}
            if col > 0 { low = low && map[row][col-1] > *point;}
            if col < col_len-1 { low = low && map[row][col+1] > *point;}
            if low {low_points.push((row,col));}
        }
    }
    low_points
}

fn risk_level (input: &str) -> i32{
    let map = input.lines().map(|line| line.chars()
                                  .map(|num| num.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
                                  .collect::<Vec<Vec<i32>>>();
    let mut risk = 0;
    let low_points = find_low_points(&map);
    for (row,col) in low_points{
        risk += 1 + map[row][col];
    }
    risk
}

fn count_basin(map : &mut Vec<Vec<(i32,bool)>>, (row,col): (usize,usize)) -> i32{
    let mut res = 1;
    let row_len = map.len();
    let col_len = map[0].len();
    map[row][col].1 = false;
    if row > 0         { if map[row-1][col].1 && map[row-1][col].0 != 9 {res += count_basin(map, (row-1,col));}}
    if row < row_len-1 { if map[row+1][col].1 && map[row+1][col].0 != 9 {res += count_basin(map, (row+1,col));}}
    if col > 0         { if map[row][col-1].1 && map[row][col-1].0 != 9 {res += count_basin(map, (row,col-1));}}
    if col < col_len-1 { if map[row][col+1].1 && map[row][col+1].0 != 9 {res += count_basin(map, (row,col+1));}}
    res
}

fn largest_basins(input: &str) -> i32{
    let mut basins = Vec::new();
    let mut map = input.lines().map(|line| line.chars()
                                  .map(|num| (num.to_digit(10).unwrap() as i32,true)).collect::<Vec<(i32,bool)>>())
                                  .collect::<Vec<Vec<(i32,bool)>>>();
    let lowest_points = find_low_points(&(map.iter_mut()
									      .map(|row| row.iter_mut()
                                          .map(|(x,_)| *x).collect()).collect()));
    for point in lowest_points{
        basins.push(count_basin(&mut map, point));
    }
    basins.sort_by(|a,b| b.cmp(a));
    basins.iter().take(3).fold(1, |acc , x| acc * x)
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day9.txt").unwrap();
    println!("Risk level           -> {}",risk_level(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day9.txt").unwrap();
    println!("Three largest basins -> {}",largest_basins(&input));
}