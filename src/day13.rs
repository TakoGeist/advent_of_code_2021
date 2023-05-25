
fn print_grid(grid: &Vec<Vec<bool>>){
    for i in grid{
        for x in i{
            if *x {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        print!("\n");
    }
}

fn fold_grid(fold_list: &Vec<(bool,i32)>, grid: &mut Vec<Vec<bool>>, steps: usize){
    for (counter,fold) in fold_list.iter().enumerate(){
        if counter==steps {return;}
        if fold.0{
            for rind in fold.1 as usize..grid.len(){
                for rcol in 0..grid[0].len(){
                    if !grid[rind][rcol] {
                        grid[2*fold.1 as usize - rind][rcol] = false;
                    }
                }
            }
            grid.truncate(fold.1 as usize);
        }
        else{
            for rind in 0..grid.len(){
                for rcol in fold.1 as usize..grid[0].len(){
                    if !grid[rind][rcol] {
                        grid[rind][2*fold.1 as usize - rcol] = false;
                    }
                }
            }
            for x in grid.iter_mut(){
                x.truncate(fold.1 as usize);
            }
        }
    }
}

fn count_dots(input: &str) -> i32{
    let (dots,folds) = input.split_once("\r\n\r\n").unwrap();
    // y == true ; x == false
    let fold_list = folds.lines().map(|x| x.split(' ').last().unwrap())
    .map(|x| x.split_once('=').unwrap())
    .fold(Vec::new(), |mut acc , (dir,num)| {
        acc.push((dir == "y",num.parse::<i32>().unwrap()));
        acc
    });
    let mut grid = dots.lines().map(|x| x.split_once(',').unwrap())
    .fold(vec![vec![true;1311];895], |mut acc , (x,y)|{
        acc[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = false;
        acc
    });
    fold_grid(&fold_list, &mut grid,1);
    let res = grid.iter().map(|x| x.iter()
                  .fold(0, |acc , x| {
                      if !x { acc + 1}
                      else {acc}
                  })).sum();
    res
}

fn get_grid(input: &str) -> Vec<Vec<bool>>{
    let (dots,folds) = input.split_once("\r\n\r\n").unwrap();
    // y == true ; x == false
    let fold_list = folds.lines().map(|x| x.split(' ').last().unwrap())
    .map(|x| x.split_once('=').unwrap())
    .fold(Vec::new(), |mut acc , (dir,num)| {
        acc.push((dir == "y",num.parse::<i32>().unwrap()));
        acc
    });
    let mut grid = dots.lines().map(|x| x.split_once(',').unwrap())
    .fold(vec![vec![true;1311];895], |mut acc , (x,y)|{
        acc[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = false;
        acc
    });
    fold_grid(&fold_list, &mut grid, fold_list.len());
    grid
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("First fold gives -> {} dots",count_dots(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("↓ Infrared Thermal Imaging Camera Code ↓");
    print_grid(&get_grid(&input));
}