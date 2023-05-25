use std::time::Instant;

const SIZES: usize = 2;
const SIZEL: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pods{
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Pods{
    fn from_char(input: char) -> Option<Pods>{
        match input{
            'A' => Some(Pods::Amber),
            'B' => Some(Pods::Bronze),
            'C' => Some(Pods::Copper),
            'D' => Some(Pods::Desert),
            _ => None
        }
    }

    fn cost(&self) -> usize {
        match self{
            Self::Amber  => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
    
    fn from_ind(input: usize) -> Option<Pods>{
        match input{
            1 => Some(Pods::Amber),
            2 => Some(Pods::Bronze),
            3 => Some(Pods::Copper),
            4 => Some(Pods::Desert),
            _ => None
        }
    }

    fn room(&self) -> usize {
        match self{
            Self::Amber  => 2,
            Self::Bronze => 4,
            Self::Copper => 6,
            Self::Desert => 8,
        }
    }

    fn _print(&self){
        match self{
            Self::Amber  => print!("A"),
            Self::Bronze => print!("B"),
            Self::Copper => print!("C"),
            Self::Desert => print!("D"),
        }
    }

}

fn range(start: usize, end: usize) -> std::ops::Range<usize>{
    if start > end {return end..(start-1)}
    start+1..end
}

fn energy(input: &str) -> usize{
    let mut hall:[Option<Pods>;11] = [None;11];
    let mut rooms = input.lines().rev().skip(1).enumerate()
            .filter(|(x,_)| *x < SIZES).fold(vec![Vec::with_capacity(SIZES);4], |mut rooms, (_,line)|{
            for (ind,x) in line.chars().filter(|x| *x == 'A' || *x == 'B' || *x == 'C' || *x == 'D')
            .enumerate(){rooms[ind].push(Pods::from_char(x).unwrap())}
            rooms
        });
    let mut res = usize::MAX;
    solve_vec(&mut hall, &mut rooms, 0, &mut res);
    res
}

fn _print_grid_vec(hall: &[Option<Pods>;11], rooms: &Vec<Vec<Pods>>){
    println!("#############");
    print!("#");
    for i in hall{
        match i {
            Some(Pods::Amber) => print!("A"),
            Some(Pods::Bronze) => print!("B"),
            Some(Pods::Copper) => print!("C"),
            Some(Pods::Desert) => print!("D"),
            None => print!("."),
        }
    }
    print!("#\n");
    print!("  #");
    for i in rooms{
            match i.len() {
                4 => i[3]._print(),
                3 => print!("."),
                2 => print!("."),
                1 => print!("."),
                _ => print!(".")
            }
            print!("#");
    }
    print!("\n  #");
    for i in rooms{
        match i.len() {
            4 => i[2]._print(),
            3 => i[2]._print(),
            2 => print!("."),
            1 => print!("."),
            _ => print!("."),
        }
        print!("#");
    }
    print!("\n  #");
    for i in rooms{
        match i.len() {
            4 => i[1]._print(),
            3 => i[1]._print(),
            2 => i[1]._print(),
            _ => print!(".")
        }
        print!("#");
    }
    print!("\n  #");
    for i in rooms{
        match i.len() {
            4 => i[0]._print(),
            3 => i[0]._print(),
            2 => i[0]._print(),
            1 => i[0]._print(),
            _ => print!("."),
        }
        print!("#");
    }
    println!("\n  #########");
}

fn complete_vec(rooms: &Vec<Vec<Pods>>) -> bool{
    rooms.iter().fold(true, |acc,x| acc && x.len() == SIZES) 
    && rooms.iter().enumerate().fold(true, |acc,(ind,room)| 
    acc && room.iter().fold(true, |acc,x| acc && *x == Pods::from_ind(ind+1).unwrap()))
}

fn solve_vec(hall: &mut[Option<Pods>;11], rooms: &mut Vec<Vec<Pods>>, cost: usize, best_cost: &mut usize){

    if complete_vec(rooms) {*best_cost = cost; return ;}
    //Set from hall to correct room
    for row in 0..11{
        if let Some(x) = hall[row]{
            let room_row = (x.room() - 1)/2;
            if !rooms[room_row].iter().fold(true, |acc,pod| acc && x == *pod)
                && !rooms[room_row].is_empty(){continue;}
            if range(row,x.room())
                .fold(true, |acc,x| acc && hall[x] == None){
                let new_cost = cost + (usize::abs_diff(row, x.room()) + (SIZES - rooms[room_row].len()))* x.cost();
                if new_cost > *best_cost {return;}
                rooms[room_row].push(x);
                hall[row] = None;
                solve_vec(hall, rooms, new_cost, best_cost);
                hall[row] = rooms[room_row].pop();
            }
        }
    }

    let row_range = [2,4,6,8];
    //Set from room to hall
    for true_row in [2,4,6,8]{
        let row = (true_row - 1)/2;
        if rooms[row].is_empty() {continue;}
        let x = rooms[row][rooms[row].len()-1];
        if true_row == x.room() 
            && rooms[row].iter().fold(true, |acc,pod| acc && *pod == x){continue;}
        let positions = [(0..true_row).rev()
            .fold((true,Vec::new()) , |(mut acc,mut vec),x| {
            if hall[x] != None {acc = false;}
            if acc && !row_range.contains(&x) {vec.push(x)} (acc,vec)}).1,
            (true_row..11)
            .fold((true,Vec::new()) , |(mut acc,mut vec),x| {
                if hall[x] != None {acc = false;}
                if acc && !row_range.contains(&x) {vec.push(x)}(acc,vec)}).1]
            .concat();
        for new in positions{
            let new_cost = cost + (usize::abs_diff(true_row, new) + (SIZES + 1) - rooms[row].len())* x.cost();
            if new_cost > *best_cost {return;}
            hall[new] = rooms[row].pop();
            solve_vec(hall, rooms, new_cost, best_cost);
            rooms[row].push(hall[new].unwrap());
            hall[new] = None;

        }
    }

}

fn complete_vec_2(rooms: &Vec<Vec<Pods>>) -> bool{
    rooms.iter().fold(true, |acc,x| acc && x.len() == SIZEL) 
    && rooms.iter().enumerate().fold(true, |acc,(ind,room)| 
    acc && room.iter().fold(true, |acc,x| acc && *x == Pods::from_ind(ind+1).unwrap()))
}

fn solve_vec_2(hall: &mut[Option<Pods>;11], rooms: &mut Vec<Vec<Pods>>, cost: usize, best_cost: &mut usize){
    //Set from hall to correct room
    for row in 0..11{
        if let Some(x) = hall[row]{
            let room_row = (x.room() - 1)/2;
            if !rooms[room_row].iter().fold(true, |acc,pod| acc && x == *pod)
                && !rooms[room_row].is_empty() {continue;}
            if range(row,x.room())
                .fold(true, |acc,x| acc && hall[x] == None){
                let new_cost = cost + (usize::abs_diff(row, x.room()) + (SIZEL - rooms[room_row].len()))* x.cost();
                if new_cost> *best_cost {return;}
                rooms[room_row].push(x);
                hall[row] = None;
                if complete_vec_2(rooms){*best_cost = new_cost;}
                else{solve_vec_2(hall, rooms, new_cost, best_cost)};
                hall[row] = rooms[room_row].pop();
            }
        }
    }

    let row_range = [2,4,6,8];
    //Set from room to hall
    for true_row in [2,4,6,8]{
        let row = (true_row - 1)/2;
        if rooms[row].is_empty() {continue;}
        let x = rooms[row][rooms[row].len()-1];
        if true_row == x.room() && rooms[row].iter().fold(true, |acc,pod| acc && *pod == x){continue;}
        let positions = [(0..true_row).rev()
            .fold((true,Vec::new()) , |(mut acc,mut vec),x| {
            if hall[x] != None {acc = false;}
            if acc && !row_range.contains(&x) {vec.push(x)} (acc,vec)}).1,
            (true_row..11)
            .fold((true,Vec::new()) , |(mut acc,mut vec),x| {
                if hall[x] != None {acc = false;}
                if acc && !row_range.contains(&x) {vec.push(x)}(acc,vec)}).1]
            .concat();
        for new in positions{
            let new_cost = cost + (usize::abs_diff(true_row, new) + SIZEL - rooms[row].len() + 1)* x.cost();
            if new_cost> *best_cost {return;}
            hall[new] = rooms[row].pop();
            if complete_vec_2(rooms){*best_cost = new_cost;}
            else{ solve_vec_2(hall, rooms, new_cost, best_cost)};
            rooms[row].push(hall[new].unwrap());
            hall[new] = None;
        }
    }
}

fn big_energy (input: &str) -> usize{
    let mut hall:[Option<Pods>;11] = [None;11];
    let mut rooms = input.lines().rev().skip(1).enumerate()
            .filter(|(x,_)| *x < SIZEL).fold(vec![Vec::with_capacity(SIZEL);4], |mut rooms, (_,line)|{
            for (ind,x) in line.chars().filter(|x| *x == 'A' || *x == 'B' || *x == 'C' || *x == 'D')
            .enumerate(){rooms[ind].push(Pods::from_char(x).unwrap())}
            rooms
        });
    let mut res = usize::MAX;
    solve_vec_2(&mut hall, &mut rooms, 0, &mut res);
    res
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day23.txt").unwrap();
    let now = Instant::now();
    println!("Least Amount of Energy -> {} in {} ms", energy(&input),now.elapsed().as_millis());
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day23.txt").unwrap();
    let now = Instant::now();
    let input = format!("{}  #D#C#B#A#\n  #D#B#A#C#\n{}",
        input.lines().take(3).map(|x| format!("{}\n",x)).collect::<String>(),
        input.lines().skip(3).map(|x| format!("{}\n",x)).collect::<String>());
    println!("Least Amount of Energy -> {} in {} ms",big_energy(&input),now.elapsed().as_millis());
}