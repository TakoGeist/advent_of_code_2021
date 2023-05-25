
fn set_digits(input :&str) -> [&str;10]{
    let mut digits = ["";10];
    let line_input = input.split(' ').filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    let mut len_5: Vec<&str> = Vec::new();
    let mut len_6: Vec<&str> = Vec::new();
    for num in line_input{
        match num.len(){
            2 => digits[1] = num, //Digit 1
            3 => digits[7] = num, //Digit 7
            4 => digits[4] = num, //Digit 4
            5 => len_5.push(num),
            6 => len_6.push(num),
            7 => digits[8] = num, //Digit 8
            _ => (),   
        }
    }
    for num in len_6{
        if !digits[1].chars().fold(true, |acc , x| acc && num.contains(x)){
            digits[6] = num;
        }
        else if digits[4].chars().fold(true, |acc , x| acc && num.contains(x)) {
            digits[9] = num;
        }
        else {digits[0] = num;}
    }
    for num in len_5{
        if num.chars().fold(true, |acc , x| acc && digits[6].contains(x)){
            digits[5] = num;
        }
        else if num.chars().fold(true, |acc , x| acc && digits[9].contains(x)) {
            digits[3] = num;
        }
        else {digits[2] = num;}
    }
    digits
}

fn output_value(input: &str, digits: [&str;10]) -> i32{
    let mut res = 0;
    for num in input.split(' ').filter(|x| !x.is_empty()){
        for (ind,code) in digits.into_iter().enumerate(){
            if num.chars().fold(true, |acc , x| acc && code.contains(x)) 
            && code.chars().fold(true, |acc , x| acc && num.contains(x)){
                res *= 10;
                res += ind as i32;
            }
        }
    }
    res
}

fn decoder(input: &String) -> i32{
    let mut res = 0;
    let x = input.lines().map(|x| x.split_once('|').unwrap());
    for line in x{
        res += output_value(line.1, set_digits(line.0));
    }
    res
}



fn count_output(input: &String) -> i32{
    let mut count = 0;
    for line_output in input.lines().map(|x| x.split_once('|').unwrap().1).filter(|x| !x.is_empty()){
        for num in line_output.split(' ').map(|x| x.trim()).filter(|x| !x.is_empty()){
            let length = num.len();
            match length{
                2 => count += 1, //Digit 1
                3 => count += 1, //Digit 7
                4 => count += 1, //Digit 4
                7 => count += 1, //Digit 8
                _ => (),
            }
        }
    }
    count
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day8.txt").unwrap();
    println!("Times 1,4,7,8 Appear -> {}",count_output(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day8.txt").unwrap();
    println!("Sum of Output Values -> {}",decoder(&input));
}