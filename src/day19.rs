use std::{collections::{HashSet, HashMap}, time::Instant};

#[derive(Debug, Clone, Copy)]
struct Align{
    axis: usize,
    flip: i32,
    translation: i32
}

#[derive(Debug,Clone)]
struct Scanner{
    beacons: Vec<Vec<i32>>
}

impl Scanner {
    fn new() -> Scanner{
        Scanner { beacons: Vec::new() }
    }
    fn push_str(&mut self,input: &str){
        self.beacons.push(input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>());
    }
}

fn orientation(field: &HashSet<Vec<i32>>, find: usize, mut exclusion: Vec<usize>, scanner: &Scanner) -> Option<Align>{
    exclusion.sort();
    exclusion.reverse();
    let mut axis_elem: Vec<usize> = vec![0,1,2];
    for i in exclusion{
        axis_elem.remove(i);
    }
    for axis in axis_elem{
        for flip in [1,-1]{
            let mut counter: HashMap<i32, i32> = HashMap::new();
            for elem in field.iter(){
                for cont in scanner.beacons.iter(){
                    *counter.entry(elem[find] - cont[axis] * flip).or_default() += 1;
                }
            }
            let (key,value) = counter.into_iter().fold((0,0), |mut acc , x|{
                if x.1 > acc.1{ acc.1 = x.1; acc.0 = x.0;}
                acc
            });
            if value >= 12{
                return Some(Align{axis: axis, flip: flip, translation: key});
            }
        }
    }
    None
}

fn beacons(input: &str) -> usize{
    let mut scanners = input.split("\r\n\r\n").map(
        |x| x.lines().skip(1).fold(Scanner::new(), |mut acc , x|{
            acc.push_str(x);
            acc
        })).collect::<Vec<_>>();
    let mut field:HashSet<Vec<i32>> = HashSet::from_iter((scanners.remove(0)).beacons.into_iter());
    while !(scanners.is_empty()){
        let scanner = scanners.remove(0);
        let x = orientation(&field, 0, Vec::new() , &scanner);
        if x.is_none(){scanners.push(scanner);continue;}
        let y = orientation(&field, 1, vec![x.unwrap().axis], &scanner);
        if y.is_none(){scanners.push(scanner);continue;}
        let z = orientation(&field, 2, vec![y.unwrap().axis,x.unwrap().axis] , &scanner);
        if z.is_none(){scanners.push(scanner);continue;}
        let x = x.unwrap();
        let y = y.unwrap();
        let z = z.unwrap();
        for elem in scanner.beacons.into_iter(){
            field.insert(vec![elem[x.axis] * x.flip + x.translation,elem[y.axis] * y.flip + y.translation,elem[z.axis] * z.flip + z.translation]);
        }
    }  
    field.len()
}

fn distance(input: &str) -> i32{
    let mut scanners = input.split("\r\n\r\n").map(
        |x| x.lines().skip(1).fold(Scanner::new(), |mut acc , x|{
            acc.push_str(x);
            acc
        })).collect::<Vec<_>>();
    let mut field:HashSet<Vec<i32>> = HashSet::from_iter((scanners.remove(0)).beacons.into_iter());
    let mut scanners_pos: Vec<(i32,i32,i32)> = Vec::new();
    while !(scanners.is_empty()){
        let scanner = scanners.remove(0);
        let x = orientation(&field, 0, Vec::new() , &scanner);
        if x.is_none(){scanners.push(scanner);continue;}
        let y = orientation(&field, 1, vec![x.unwrap().axis], &scanner);
        if y.is_none(){scanners.push(scanner);continue;}
        let z = orientation(&field, 2, vec![y.unwrap().axis,x.unwrap().axis] , &scanner);
        if z.is_none(){scanners.push(scanner);continue;}
        let x = x.unwrap();
        let y = y.unwrap();
        let z = z.unwrap();
        for elem in scanner.beacons.into_iter(){
            field.insert(vec![elem[x.axis] * x.flip + x.translation,elem[y.axis] * y.flip + y.translation,elem[z.axis] * z.flip + z.translation]);
        }
        scanners_pos.push((x.translation,y.translation,z.translation));
    }
    let mut res = 0;
    for ind in 0..scanners_pos.len(){
        let outer = scanners_pos[ind];
        for elem in ind..scanners_pos.len(){
            let inner = scanners_pos[elem];
            let distance = i32::abs(outer.0-inner.0) + i32::abs(outer.1-inner.1) + i32::abs(outer.2-inner.2);
            if distance > res {res = distance;}
        }        
    }
    res
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day19.txt").unwrap();
    let now = Instant::now();
    println!("Total Number of Beacons      -> {} in {} ms",beacons(&input),now.elapsed().as_millis());
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day19.txt").unwrap();
    let now = Instant::now();
    println!("Max Distance between Beacons -> {} in {} ms",distance(&input),now.elapsed().as_millis());
}