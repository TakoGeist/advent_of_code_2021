use std::collections::HashMap;

const STEPS_1:usize = 10;
const STEPS_2:usize = 40;

fn polymerization(input: &str) -> i32{
    let (template,insertions) = input.split_once("\r\n\r\n").unwrap();
    let pairs = insertions.lines()
    .map(|x| x.split_once(" -> ").unwrap()).fold(HashMap::new(), |mut acc, (x,y)| {
        acc.insert((x.chars().nth(0).unwrap(),x.chars().nth(1).unwrap()), y.chars().nth(0).unwrap());
        acc
    });
    let mut template = template.chars().collect::<Vec<char>>();
    let mut insertion:Vec<(usize,char)> = Vec::new();
    for _ in 0..STEPS_1{
        for ind in 1..template.len(){
            insertion.push((ind, *pairs.get(&(template[ind-1],template[ind])).unwrap()));
        }
        insertion.reverse();
        for (ind,elem) in insertion.iter(){
            template.insert(*ind,*elem);
        }
        insertion.clear();
    }
    let res = template.iter().fold(vec![0;26], |mut acc , x| {
        acc[(*x as u8 - b'A') as usize] += 1;
        acc
    });
    res.iter().max().unwrap() - res.iter().filter(|x| **x != 0).min().unwrap()
}

fn efficient_polymerization(input: &str) -> usize{
    let (template,insertions) = input.split_once("\r\n\r\n").unwrap();
    let pairs = insertions.lines()
    .map(|x| x.split_once(" -> ").unwrap()).fold(HashMap::new(), |mut acc, (x,y)| {
        acc.insert((x.chars().nth(0).unwrap(),x.chars().nth(1).unwrap()), y.chars().nth(0).unwrap());
        acc
    });
    let mut poly: HashMap<(char,char), usize> = HashMap::new();
    let template = template.chars().collect::<Vec<char>>();
    for ind in 1..template.len(){
        *poly.entry((template[ind-1],template[ind])).or_default() += 1;
    }
    for _ in 0..STEPS_2{
        poly = poly.into_iter().fold( HashMap::new(), |mut poly  , (pair, count)|{
            *poly.entry((pair.0,*pairs.get(&pair).unwrap())).or_default() += count;
            *poly.entry((*pairs.get(&pair).unwrap(),pair.1)).or_default() += count;
            poly
        });
    }
    let mut counter = poly.into_iter()
    .fold(HashMap::<char,usize>::new(), |mut counter, ((x,_),num)|{
        *counter.entry(x).or_default() += num;
        counter
    });
    *counter.entry(template.into_iter().last().unwrap()).or_default() += 1;
    let mut counter = counter.into_iter().collect::<Vec<_>>();
    counter.sort_by(|(_,a) , (_,b)| a.cmp(b));
    counter[counter.len()-1].1 - counter[0].1
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    println!("After 10 steps -> {}",polymerization(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    println!("After 40 steps -> {}",efficient_polymerization(&input));
}