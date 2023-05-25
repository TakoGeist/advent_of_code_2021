use std::{collections::HashSet, time::Instant};

const ENH_PART1: u8 = 2;
const ENH_PART2: u8 = 50;

fn process(input: &str, times: u8) -> usize{
    let input = input.split_once("\r\n\r\n").unwrap();
    let mut x_range = 0..input.1.lines().nth(0).unwrap().chars().count() as isize;
    let mut y_range = 0..input.1.lines().count() as isize;
    let code = input.0.chars().map(|x| x=='#').collect::<Vec<_>>();
    let mut image = input.1.lines().enumerate().map(|(y,line)| 
            line.chars().enumerate().map(move |(x,ch)| if ch=='#' {Some((x as isize,y as isize))} else{None}))
            .flatten().flatten().collect::<HashSet<_>>();
    let neighbours = [(-1,-1),(0,-1),(1,-1),(-1,0),(0,0),(1,0),(-1,1),(0,1),(1,1)];
    for cycle in 0..times{
        let mut cl_image = HashSet::new();
        let xn_range = (x_range.start-1)..(x_range.end+1);
        let yn_range = (y_range.start-1)..(y_range.end+1);
        for y in yn_range.clone(){
            for x in xn_range.clone(){
                let mut index = 0;
                for (xn, yn) in neighbours.iter().map(|(a,b)| (x+a,y+b)){
                    index <<= 1;
                    if x_range.contains(&xn) && y_range.contains(&yn){
                        if image.contains(&(xn,yn)){
                            index += 1;
                        }
                    }
                    else if code[0] && cycle % 2 == 1 {
                        index += 1;
                    }
                }
                if code[index]{
                    cl_image.insert((x,y));
                }
            }
        }
        image = cl_image;
        x_range = xn_range;
        y_range = yn_range;
    }
    image.len()
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day20.txt").unwrap();
    println!("Lit Pixels After {} Enhancements  -> {}",ENH_PART1,process(&input,ENH_PART1));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day20.txt").unwrap();
    let now = Instant::now();
    println!("Lit Pixels After {} Enhancements -> {} in {} ms",ENH_PART2,process(&input,ENH_PART2),now.elapsed().as_millis());
}
