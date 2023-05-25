
struct Line{
    beg: Coord,
    end: Coord,
}

impl Line {
    fn new(input: &str) -> Line{
        let (beg,end) = input.split_once("->").unwrap();
        Line { beg: Coord::new(beg), end: Coord::new(end) }
    }

    fn ord(&mut self){
        if self.beg.x > self.end.x{
            (self.beg.x,self.beg.y,self.end.x,self.end.y) = (self.end.x,self.end.y,self.beg.x,self.beg.y);
        }
    }

    fn is_horizontal(&self) -> bool{
        self.beg.y == self.end.y
    }

    fn is_vertical(&self) -> bool{
        self.beg.x == self.end.x
    }

    fn is_diagonal(&self) -> bool{
        i32::abs(self.beg.x-self.end.x) == i32::abs(self.beg.y-self.end.y)
    }    
}
#[derive(Debug)]
struct Coord{
    x: i32,
    y: i32,
}

impl Coord{
    fn new(input: &str) -> Coord{
        let (x,y) = input.split_once(",").unwrap();
        Coord{
            x: x.trim().parse::<i32>().unwrap(),
            y: y.trim().parse::<i32>().unwrap(),
        }
    }
}

fn field_data(field: &mut Vec<Vec<i32>>, lines: &[Line]){
    for vent in lines{
        if vent.is_horizontal(){
            let row = vent.beg.y as usize;
            if vent.beg.x < vent.end.x{
                for col in vent.beg.x..=vent.end.x{
                    field[row][col as usize] += 1;
                }
            }else{
                for col in vent.end.x..=vent.beg.x{
                    field[row][col as usize] += 1;
                }
            }
        }
        else if vent.is_vertical(){
            let col = vent.beg.x as usize;
            if vent.beg.y < vent.end.y{
                for row in vent.beg.y..=vent.end.y{
                    field[row as usize][col] += 1;
                }
            }else{
                for row in vent.end.y..=vent.beg.y{
                    field[row as usize][col] += 1;
                }
            }
        }
    }
}

fn field_diagonal(field: &mut Vec<Vec<i32>>, lines: &[Line]){
    for vent in lines{
        if vent.is_diagonal(){
            if vent.beg.x==vent.beg.y{
                if vent.beg.y < vent.end.y{
                    for ind in vent.beg.y as usize..=vent.end.y as usize{
                        field[ind][ind] += 1;
                    }
                }else{
                    for ind in vent.end.y as usize..=vent.beg.y as usize{
                        field[ind][ind] += 1;
                    }
                }
            }
                else {
                    if vent.beg.y > vent.end.y{
                        for ind in 0..=vent.beg.y-vent.end.y{
                            field[(vent.beg.y-ind) as usize][(vent.beg.x+ind) as usize] += 1;
                        }
                    }else{
                        for ind in 0..=vent.end.y-vent.beg.y{
                            field[(vent.beg.y+ind) as usize][(vent.beg.x+ind) as usize] += 1;
                        }
                    }
            }
        }
    }
}

fn point_overlap(input: &String) -> (i32,i32){
    let mut count = 0;
    let mut lines: Vec<Line> = Vec::new();
    let mut field = vec![vec![0;1000];1000];
    for line in input.lines(){
        lines.push(Line::new(line));
    }
    for line in lines.iter_mut(){
        line.ord();
    }

    field_data(&mut field, &lines);
    
    for row in field.iter_mut(){
        for elem in row{
            if *elem > 1{
                count += 1;
            }
        }
    }
    (count,point_overlap_diagonal(&mut field, &lines))
}

fn point_overlap_diagonal(field: &mut Vec<Vec<i32>>,lines: &[Line]) -> i32{
    let mut count = 0;
    field_diagonal(field, lines);
    for row in field.iter_mut(){
        for elem in row{
            if *elem > 1{
                count += 1;
            }
        }
    }
    count
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day5.txt").unwrap();
    let (res,_) = point_overlap(&input);
    println!("Count without diagonals -> {}",res);
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day5.txt").unwrap();
    let (_,res) = point_overlap(&input);
    println!("Count with diagonals    -> {}",res);
}