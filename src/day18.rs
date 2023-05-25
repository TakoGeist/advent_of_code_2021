
fn add(homework: &mut Vec<(i32,u32)>, mut line: Vec<(i32,u32)>){
    homework.append(&mut line);
    for elem in homework.iter_mut(){
        elem.0 += 1;
    }
}

fn reduce(homework: &mut Vec<(i32,u32)>){
    for ind in 0..homework.len()-1{
        if homework[ind].0 > 4{
            let (left,right) = (homework[ind].1,homework[ind+1].1);
            homework.remove(ind+1);
            homework[ind] = (homework[ind].0-1,0);
            if ind>0{homework[ind-1].1 += left;}
            if ind<homework.len()-1{homework[ind+1].1 += right};
            return reduce(homework);
        }
    }

    for ind in 0..homework.len(){
        if homework[ind].1 > 9{
            let (depth,value) = homework[ind];
            let half = value / 2;
            homework[ind] = (depth+1,half);
            homework.insert(ind+1, (depth+1,value - half));
            return reduce(homework);
        }
    }
}

fn magnitude(homework: &mut Vec<(i32,u32)>) -> u32{
    if homework.len()!=1{
        for ind in 1..homework.len(){
            if homework[ind].0 == homework[ind-1].0{
                let (depth,mut value) =homework.remove(ind);
                value = homework[ind-1].1 * 3 + value * 2;
                homework[ind-1] = (depth-1,value);
                return magnitude(homework);
            }
        }
    }
    homework[0].1
}

fn sum_magnitude(input : &str) -> u32{
    let mut homework = input.lines().map(|x| x.chars()
            .fold((0,Vec::new()), |(mut depth , mut acc) , x|{
                match x{
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => (),
                    _ => acc.push((depth,x.to_digit(10).unwrap())),
                }
                (depth,acc)
            }).1).collect::<Vec<_>>();

    while homework.len() > 1{
        let line = homework.remove(1);
        add(&mut homework[0], line);
        reduce(&mut homework[0]);
    }
    magnitude(&mut homework[0])
}

fn largest_magnitude(input: &str) -> u32{
    let homework = input.lines().map(|x| x.chars()
            .fold((0,Vec::new()), |(mut depth , mut acc) , x|{
                match x{
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => (),
                    _ => acc.push((depth,x.to_digit(10).unwrap())),
                }
                (depth,acc)
            }).1).collect::<Vec<_>>();
    let mut max = 0;
    for ind in 0..homework.len(){
        for sec in ind..homework.len(){
            let mut first = homework[ind].clone();
            let second = homework[sec].clone();
            add(&mut first,second);
            reduce(&mut first);
            let temp = magnitude(&mut first);
            if max < temp {max = temp;}
            let first = homework[ind].clone();
            let mut second = homework[sec].clone();
            add(&mut second,first);
            reduce(&mut second);
            let temp = magnitude(&mut second);
            if max < temp {max = temp;}
        }
    }
    max
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day18.txt").unwrap();
    println!("Magnitude of Final Sum -> {}",sum_magnitude(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day18.txt").unwrap();
    println!("Largest Magnitude of Two Sums -> {}",largest_magnitude(&input));
}