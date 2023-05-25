
fn from_str(input: &str) -> [[isize;2];3]{
    let mut input = input.split(",").map(|x| x.split_once("=").unwrap().1)
    .map(|x| x.split_once("..").unwrap());
    let x = input.next().unwrap();
    let y = input.next().unwrap();
    let z = input.next().unwrap();
    [[x.0.parse::<isize>().unwrap(),x.1.parse::<isize>().unwrap()], 
     [y.0.parse::<isize>().unwrap(),y.1.parse::<isize>().unwrap()], 
     [z.0.parse::<isize>().unwrap(),z.1.parse::<isize>().unwrap()]] 
}

fn intersect(cube: &[[isize;2];3], cuboid: &[[isize;2];3]) -> Option<[[isize;2];3]>{
    Some([[
        cube[0][0].max(cuboid[0][0]),
        cube[0][1].min(cuboid[0][1]),
    ],[
        cube[1][0].max(cuboid[1][0]),
        cube[1][1].min(cuboid[1][1]),
    ],[
        cube[2][0].max(cuboid[2][0]),
        cube[2][1].min(cuboid[2][1]),
    ]]).filter(|x| x.iter().all(|l| l[0] <= l[1]))
}

fn remove(cube: &[[isize;2];3], cuboid: [[isize;2];3], add: &mut Vec<[[isize;2];3]>){
    if cube[0][0] < cuboid[0][0]{
        add.push([[cube[0][0],cuboid[0][0] - 1], [cube[1][0],cube[1][1]],[cube[2][0],cube[2][1]]])
    }
    if cube[1][0] < cuboid[1][0]{
        add.push([[cuboid[0][0],cuboid[0][1]],[cube[1][0],cuboid[1][0] - 1],[cube[2][0],cube[2][1]]])
    }
    if cube[2][0] < cuboid[2][0]{
        add.push([[cuboid[0][0],cuboid[0][1]],[cuboid[1][0],cuboid[1][1]],[cube[2][0],cuboid[2][0] - 1]])
    }
    if cube[0][1] > cuboid[0][1]{
        add.push([[cuboid[0][1] + 1,cube[0][1]], [cube[1][0],cube[1][1]], [cube[2][0], cube[2][1]]])
    }
    if cube[1][1] > cuboid[1][1]{
        add.push([[cuboid[0][0],cuboid[0][1]],[cuboid[1][1] + 1,cube[1][1]],[cube[2][0],cube[2][1]]])
    }
    if cube[2][1] > cuboid[2][1]{
        add.push([[cuboid[0][0],cuboid[0][1]],[cuboid[1][0],cuboid[1][1]],[cuboid[2][1] + 1,cube[2][1]]])
    }
}

fn volume(cube: &[[isize;2];3]) -> isize{
    (cube[0][1] - cube[0][0] + 1) * (cube[1][1] - cube[1][0] + 1) * (cube[2][1] - cube[2][0] + 1) 
}

fn initialization(input: &str) -> isize{
    let queue = &input.lines().rev().map(|x| x.split_once(' ').unwrap())
        .map(|(cond,x)| if "on" == cond {(true,from_str(x))} else {(false,from_str(x))})
        .filter(|(_,cube)| 
            cube[0][0] >= -50 && cube[0][1] <= 50 && cube[1][0] >= -50 && cube[1][1] <= 50 && cube[2][0] >= -50 && cube[2][1] <= 50)
        .collect::<Vec<_>>();
    let mut res = 0;
    for ind in 0..queue.len(){
        if !queue[ind].0 {continue;}
        let (_,cuboid) = queue[ind].clone();
        let mut add = vec![cuboid];
        for inter in 0..ind{
            let mut aux = Vec::new();
            while !add.is_empty(){
                let cuboid = add.pop().unwrap();
                if let Some(overl) = intersect(&cuboid, &queue[inter].1){
                    remove(&cuboid,overl,&mut aux);
                }
                else {
                    aux.push(cuboid);
                }
            }
            add = aux;            
        }
        res += add.iter().fold(0, |acc,x| acc + volume(x));
    }
    res    
}

fn reboot(input: &str) -> isize{
    let queue = &input.lines().rev().map(|x| x.split_once(' ').unwrap())
        .map(|(cond,x)| if "on" == cond {(true,from_str(x))} else {(false,from_str(x))})
        .collect::<Vec<_>>();
    let mut res = 0;
    for ind in 0..queue.len(){
        if !queue[ind].0 {continue;}
        let (_,cuboid) = queue[ind].clone();
        let mut add = vec![cuboid];
        for inter in 0..ind{
            let mut aux = Vec::new();
            while !add.is_empty(){
                let cuboid = add.pop().unwrap();
                if let Some(overl) = intersect(&cuboid, &queue[inter].1){
                    remove(&cuboid,overl,&mut aux);
                }
                else {
                    aux.push(cuboid);
                }
            }
            add = aux;            
        }
        res += add.iter().fold(0, |acc,x| acc + volume(x));
    }
    res    
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day22.txt").unwrap();
    println!("On cubes after initialization -> {}",initialization(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day22.txt").unwrap();
    println!("On cubes after reboot         -> {}",reboot(&input));
}