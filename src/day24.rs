#[derive(Debug)]
struct Level{
    index: usize,
    opt: isize,
    x_const: isize,
    y_const: isize
}

fn get_const (input: Vec<&str>) -> Vec<Level>{
    let mut res = Vec::new();
    for (ind,block) in input.into_iter().enumerate(){
        let mut line = block.lines().skip(1);
        let opt = line.next().unwrap().split(' ').collect::<Vec<_>>()[2].parse().unwrap();
        let x_const = line.next().unwrap().split(' ').collect::<Vec<_>>()[2].parse().unwrap();
        let y_const = line.next().unwrap().split(' ').collect::<Vec<_>>()[2].parse().unwrap();
        res.push(Level{index: ind,opt: opt, x_const: x_const, y_const: y_const});
    }
    res
}

fn solver_big(reduce: Level, increase: Level, number: &mut [isize;14]){
    let dif = reduce.y_const + increase.x_const;
    for w in (1..10).rev(){
        if (1..10).contains(&(w + dif)){
            number[reduce.index] = w;
            number[increase.index] = w + dif;
            return;
        }
    }
}

fn solver_small(reduce: Level, increase: Level, number: &mut [isize;14]){
    let dif = reduce.y_const + increase.x_const;
    for w in 1..10{
        if (1..10).contains(&(w + dif)){
            number[reduce.index] = w;
            number[increase.index] = w + dif;
            return;
        }
    }
}

fn equations(input: Vec<Level>, solver: fn(Level,Level,&mut [isize;14])) -> isize{
    let mut number = [0;14];
    let mut stack = Vec::new();
    for step in input{
        if stack.is_empty(){stack.push(step);}
        else{
            if stack.last().unwrap().opt != step.opt{solver(stack.pop().unwrap(), step, &mut number)}
            else {stack.push(step);}
        }
    }
    let mut res = 0;
    for num in number{
        res *= 10;
        res += num;
    }
    res
}

fn model_number_big(input: &str) -> isize{
    let input = input.lines().map(|x| (x,input.lines().filter(|a| *a == x).count())).collect::<Vec<_>>();
    let input = input.iter().filter(|(a,x)| *x < 14 || a.contains("inp")).map(|(a,_)| format!("{}\n",*a) ).collect::<String>();
    let input = input.split("inp").filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let input = get_const(input);
    equations(input,solver_big)
}

fn model_number_small(input: &str) -> isize{
    let input = input.lines().map(|x| (x,input.lines().filter(|a| *a == x).count())).collect::<Vec<_>>();
    let input = input.iter().filter(|(a,x)| *x < 14 || a.contains("inp")).map(|(a,_)| format!("{}\n",*a) ).collect::<String>();
    let input = input.split("inp").filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let input = get_const(input);
    equations(input,solver_small)
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day24.txt").unwrap();
    println!("Biggest model number -> {}",model_number_big(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day24.txt").unwrap();
    println!("Smallest model number -> {}",model_number_small(&input));
}