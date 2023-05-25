
use std::time::{Instant};

#[derive(Clone, Copy)]
struct Last{
    coord: (usize,usize),
    cost: u32
}

#[derive(Clone, Copy)]
struct Cell{
    last: Option<Last>,
    risk: u32
}

fn insert_ord(path: &mut Vec<(usize,usize)>, coord: (usize,usize)){
    if path.len() == 0 {path.push(coord);return;}
    let mut count = 0;
    while count < path.len() && path[count].0 + path[count].1 < coord.0 + coord.1 {
        count += 1;
    }
    if count > path.len() {path.push(coord);}
    else {path.insert(count, coord);}
}

fn path_finder(grid: &mut Vec<Vec<Cell>>){
    let mut path: Vec<(usize,usize)> = Vec::new();
    path.push((0,0));
    let neighbors = [(-1,0),(1,0),(0,-1),(0,1)];
    let end = (grid[0].len()-1,grid.len()-1);
    let x_range = 0..=end.0;
    let y_range = 0..=end.1;
    while !path.is_empty() {
        let coord = path.remove(0);
        if coord == end {break;}
        for (x,y) in neighbors.iter().map(|(a,b)| (*a + coord.0 as i32, *b + coord.1 as i32)){
            let (x,y) = (x as usize, y as usize);
            if x_range.contains(&x) && y_range.contains(&y){
                let current_risk = grid[y][x].risk;
                let cost_so_far = grid[coord.1][coord.0].last.as_ref().unwrap().cost;
                match &grid[y][x].last{
                    Some(a) => if !(a.coord == (x,y)) && cost_so_far + current_risk < a.cost{ 
                                         grid[y][x].last = Some(Last{coord: coord, cost : cost_so_far + current_risk});
                                         insert_ord(&mut path,  (x,y));}, 
                    None => {grid[y][x].last = Some(Last{coord: coord, cost : cost_so_far + current_risk});
                             insert_ord(&mut path, (x,y));},
                }    
            } 
        }
    }
}

fn horizontal_update(grid: &mut Vec<Vec<Cell>>, val: Cell, step : usize, y: usize, x: usize){
    for a in 1..5{
        if val.risk + a > 9 {grid[y][x + (a as usize *step) as usize].risk = (val.risk + a) % 9}
        else {grid[y][x + (a as usize*step) as usize].risk = val.risk + a}
    }
}

fn vertical_update(grid: &mut Vec<Vec<Cell>>, val: Cell, step : usize, y: usize, x: usize){
    for a in 1..5{
        if val.risk + a > 9 {grid[y + (a as usize *step)][x].risk = (val.risk + a) % 9}
        else {grid[y + (a as usize *step)][x].risk = val.risk + a}
    }
}

fn risk_level(input :&str) -> u32{
    let mut grid = input.lines().map(|line| line.chars()
        .map(|x| Cell{last: None, risk:x.to_digit(10).unwrap()}).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    grid[0][0].last = Some(Last{coord: (0,0), cost: 0});
    path_finder(&mut grid);
    grid[grid.len()-1][grid.len()-1].last.as_ref().unwrap().cost
}

fn risk_level_big(input: &str) -> u32{
    let mut grid = input.lines().map(|line| line.chars()
        .map(|x| Cell{last: None, risk: x.to_digit(10).unwrap()}).collect::<Vec<_>>())
        .collect::<Vec<_>>();    
    grid[0][0].last = Some(Last{coord: (0,0), cost: 0});
    let length = grid.len();
    let new_length = 5*length;
    for row in grid.iter_mut(){
        row.resize(new_length, Cell{last: None, risk: 0});
    }
    for y in 0..length{
        for x in 0..length{
            let value = grid[y][x];
            horizontal_update(&mut grid, value, length, y, x);
        }
    }
    grid.resize(new_length, vec![Cell{last:None,risk:0};new_length]);
    for y in 0..length{
        for x in 0..new_length{
            let value = grid[y][x];
            vertical_update(&mut grid, value, length, y, x);
        }
    }
    path_finder(&mut grid);
    grid[new_length-1][new_length-1].last.as_ref().unwrap().cost
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day15.txt").unwrap();
    let now = Instant::now();
    println!("For 1 Tile Shortest Path Costs   -> {} in {} ms",risk_level(&input),now.elapsed().as_millis());
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day15.txt").unwrap();
    let now = Instant::now();
    println!("For Full Map Shortest Path Costs -> {} in {} ms",risk_level_big(&input),now.elapsed().as_millis());
}