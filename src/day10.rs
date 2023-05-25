
fn syntax_checker(input :&str) -> i32{
    let mut res = 0;
    let mut stack:Vec<char> = Vec::new();
    for line in input.lines(){
        stack.clear();
        for char in line.trim().chars(){
            match char{
                '(' => stack.push(char),
                '[' => stack.push(char),
                '{' => stack.push(char),
                '<' => stack.push(char),
                ')' => if stack.pop().unwrap() != '('{res += 3; break;}
                ']' => if stack.pop().unwrap() != '['{res += 57; break;}
                '}' => if stack.pop().unwrap() != '{'{res += 1197; break;}
                '>' => if stack.pop().unwrap() != '<'{res += 25137; break;}
                _ => (),
            }
        }
    }
    res
}

fn autocomplete(input :&str) -> i64{
    let mut res: Vec<i64> = Vec::new();
    let mut stack:Vec<char> = Vec::new();
    let mut complete:bool;
    for line in input.lines(){
        stack.clear();
        complete = true;
        for char in line.trim().chars(){
            match char{
                '(' => stack.push(char),
                '[' => stack.push(char),
                '{' => stack.push(char),
                '<' => stack.push(char),
                ')' => if stack.pop().unwrap() != '('{complete = false; break;}
                ']' => if stack.pop().unwrap() != '['{complete = false; break;}
                '}' => if stack.pop().unwrap() != '{'{complete = false; break;}
                '>' => if stack.pop().unwrap() != '<'{complete = false; break;}
                _ => (),
            }
        }
        if complete{
            let mut x = 0;
            stack.reverse();
            for left in stack.iter(){
                x *= 5;
                match left{
                    '(' => x += 1,
                    '[' => x += 2,
                    '{' => x += 3,
                    '<' => x += 4,
                    _ => (),
                }
            }
            res.push(x);
        }
    }
    res.sort();
    *res.iter().nth(res.len()/2).unwrap()
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Syntax error score -> {}",syntax_checker(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Autocomplete score -> {}",autocomplete(&input));
}