use std::collections::HashMap;

const WINNING_SCORE_1: usize = 1000;
const WINNING_SCORE_2: usize = 21;

#[derive(Debug,PartialEq, Eq, Hash)]
struct Play{
    player_1: usize,
    score_1: usize,
    player_2: usize,
    score_2: usize,
}

fn practice(input: &str) -> usize{
    let (player_1,player_2) = input.split_once("\r\n").unwrap();
    let mut player_1 = player_1.split(" ").last().unwrap().parse::<usize>().unwrap(); 
    let mut player_2 = player_2.split(" ").last().unwrap().parse::<usize>().unwrap();
    let mut dice = 1;
    let mut score_1 = 0; 
    let mut score_2 = 0;
    loop{
        player_1 = (player_1 + 3 * dice + 2) % 10 + 1;
        dice += 3;
        score_1 += player_1;
        if score_1 >= WINNING_SCORE_1 {return score_2 * (dice-1);}
        player_2 = (player_2 + (3 * dice + 3) - 1) % 10 + 1;
        dice += 3;
        score_2 += player_2;
        if score_2 >= WINNING_SCORE_1 {return score_1 * (dice-1);}
    } 
}

fn game(input: &str) -> usize{
    let rolls = [1,3,6,7,6,3,1];
    let (player_1,player_2) = input.split_once("\r\n").unwrap();
    let player_1 = player_1.split(" ").last().unwrap().parse::<usize>().unwrap(); 
    let player_2 = player_2.split(" ").last().unwrap().parse::<usize>().unwrap();
    let mut res = [0,0];
    let mut state = HashMap::new();
    state.insert(Play{player_1: player_1,score_1:0,player_2:player_2,score_2:0}, 1);
    while !state.is_empty(){
        let mut new_state: HashMap<Play, usize> = HashMap::new();
        for update in state.into_iter(){
            if update.0.score_1 >= WINNING_SCORE_2 {res[0] += update.1;continue;} 
            if update.0.score_2 >= WINNING_SCORE_2 {res[1] += update.1;continue;} 
            for (dice,roll) in rolls.iter().enumerate(){
                let player_1 = (update.0.player_1 + dice + 2) % 10 + 1;
                let play = Play{player_1 : player_1, score_1 : update.0.score_1 + player_1,
                    player_2 : update.0.player_2, score_2 : update.0.score_2};
                    *new_state.entry(play).or_insert(0) += update.1 * *roll;
                }
        }
        let mut new_2state: HashMap<Play, usize> = HashMap::new();
        for update in new_state.into_iter(){
            if update.0.score_1 >= WINNING_SCORE_2 {res[0] += update.1;continue;} 
            if update.0.score_2 >= WINNING_SCORE_2 {res[1] += update.1;continue;} 
            for (dice,roll) in rolls.iter().enumerate(){
                let player_2 = (update.0.player_2 + dice + 2) % 10 + 1;
                let play = Play{player_1 : update.0.player_1, score_1 : update.0.score_1,
                                      player_2 : player_2, score_2 : update.0.score_2 + player_2};
                *new_2state.entry(play).or_insert(0) += update.1 * *roll;
            }
        }
        state = new_2state;
    }
    *res.iter().max().unwrap()  
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day21.txt").unwrap();
    println!("Multiply Num of Rolls by Losing Score -> {}",practice(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day21.txt").unwrap();
    println!("Num of Universes - Player that Wins The Most -> {}",game(&input));
}