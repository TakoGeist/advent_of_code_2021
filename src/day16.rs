use std::collections::VecDeque;

#[derive(Default)]
struct Transmission{
    message: VecDeque<bool>,
}

struct Packet{
    type_id: i64,
    value: Option<i64>,
    subpacket: Option<Vec<Packet>>,
}

impl Transmission{
    fn push(&mut self, bit: char){
        for elem in match bit{
            '0' => [false,false,false,false],
            '1' => [false,false,false,true],
            '2' => [false,false,true,false],
            '3' => [false,false,true,true],
            '4' => [false,true,false,false],
            '5' => [false,true,false,true],
            '6' => [false,true,true,false],
            '7' => [false,true,true,true],
            '8' => [true,false,false,false],
            '9' => [true,false,false,true],
            'A' => [true,false,true,false],
            'B' => [true,false,true,true],
            'C' => [true,true,false,false],
            'D' => [true,true,false,true],
            'E' => [true,true,true,false],
            'F' => [true,true,true,true],
            _ => panic!(""),
        }.into_iter(){
            self.message.push_back(elem);
        }
    }

    fn get(&mut self, length: usize) -> i64{
        if self.message.len() < length {return 0;}
        let mut res = 0;
        for _ in 0..length{
            res <<= 1;
            if self.message.pop_front().unwrap(){
                res += 1;
            }
        }
        res
    }
}

fn get_packet(message: &mut Transmission) -> (i64, usize){
    let mut res = 0;
    let mut count = 0;
    let version = message.get(3);
    res += version;
    let type_id = message.get(3);
    if type_id == 4{
        count += 6;
        loop{
            count += 5;
            if message.get(5) < 16 {break;}
        }
    }
    else{
        if message.message.pop_front().unwrap(){
            count += 18;
            for _ in 0..message.get(11){
                let (x,y) = get_packet(message);
                res += x;
                count += y;
            }
        }
        else{
            count += 22;
            let mut length = message.get(15);
            while length > 0{
                let (x,y) = get_packet(message);
                res += x;
                count += y;
                length -= y as i64;
            }
        }
    }    
    (res,count)
}

fn get_value(message: &mut Transmission) -> (Packet, usize){
    let mut res;
    let mut count = 0;
    let _version = message.get(3);
    let type_id = message.get(3);
    if type_id == 4 {
        count += 6;
        let mut val = 0;
        loop{
            let group = message.get(5);
            val <<= 4;
            count += 5;
            if group >= 16 {val += group - 16;}
            else {val += group; break;}
        }
        res = Packet{type_id: type_id, value: Some(val),subpacket: None}; 
        
    }
    else{
        res = Packet{type_id: type_id, value: None, subpacket: Some(Vec::new())};
        let mut temp = Vec::new();
        if message.message.pop_front().unwrap(){
            count += 18;
            for _ in 0..message.get(11){
                let (x,y) = get_value(message);
                temp.push(x);
                count += y;
            }
        }
        else{
            count += 22;
            let mut length = message.get(15);
            while length > 0{
                let (x,y) = get_value(message);
                temp.push(x);
                count += y;
                length -= y as i64;
            }
        }
        res.subpacket.as_mut().unwrap().append(&mut temp);
    }
    (res,count)
}

fn decode(packet: &Packet) -> i64{
    let mut res = 0;
    let subpacket = packet.subpacket.as_ref();
    match packet.type_id{
        0 => res = subpacket.unwrap().into_iter().map(|x| decode(&x)).sum(),
        1 => res = subpacket.unwrap().into_iter().map(|x| decode(&x)).product(),
        2 => res = subpacket.unwrap().into_iter().map(|x| decode(&x)).min().unwrap(),
        3 => res = subpacket.unwrap().into_iter().map(|x| decode(&x)).max().unwrap(),
        4 => res = packet.value.unwrap(),
        5 => if decode(&subpacket.unwrap()[0]) >  decode(&subpacket.unwrap()[1]) {res = 1},
        6 => if decode(&subpacket.unwrap()[0]) <  decode(&subpacket.unwrap()[1]) {res = 1},
        7 => if decode(&subpacket.unwrap()[0]) == decode(&subpacket.unwrap()[1]) {res = 1},
        _ => (),
    }
    res
}

fn version_sum(input :&str) -> i64{
    let mut message = Transmission::default();
    for bit in input.trim().chars(){
        message.push(bit);
    }
    get_packet(&mut message).0
}

fn calculate(input :&str) -> i64{
    let mut message = Transmission::default();
    for bit in input.trim().chars(){
        message.push(bit);
    }
    let (packet,_) = get_value(&mut message);
    decode(&packet)
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day16.txt").unwrap();
    println!("Sum of versions equals -> {}",version_sum(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day16.txt").unwrap();
    println!("Expresion evaluates to -> {}",calculate(&input));
}