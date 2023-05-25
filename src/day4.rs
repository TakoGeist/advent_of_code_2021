
struct Bingo{
    board: Vec<Vec<(i32,bool)>>,
    row: Vec<i32>,
    column: Vec<i32>,
    turn: i32,
}

impl Bingo{
    fn new(input: &String) -> Bingo{
        Bingo { board: input.lines().map(|x| x.split(' ')
                            .filter(|x| *x != "").into_iter()
                            .map(|x| (x.trim().parse::<i32>().unwrap(),false))
                            .collect::<Vec<(i32,bool)>>())
                            .collect::<Vec<Vec<(i32,bool)>>>(),
                row: vec![0;5],
                column: vec![0;5],
                turn: 0,
        }
    }

    fn win_turn(&mut self, numbers: &Vec<i32>){
        let mut turn = 0;
        for num in numbers{
            turn += 1;
            for (row,relem) in self.board.iter_mut().enumerate(){
                for (col,celem) in relem.iter_mut().enumerate(){
                    if celem.0 == *num {
                        celem.1 = true;
                        self.row[row] += 1;
                        self.column[col] += 1;
                    }
                }
            }
            if self.column.iter_mut().any(|x| *x == 5) || self.row.iter_mut().any(|x| *x == 5) {
                self.turn = turn;
                return;
            } 
        }
    }

    fn score(&self, numbers: &Vec<i32>) -> i32{
        let mut sum = 0;
        for row in self.board.iter(){
            for num in row{
                if !num.1{
                    sum += num.0;
                }
            }
        }
        sum * numbers[(self.turn-1) as usize]
    }

}

fn fst_winner(boards: &Vec<Bingo>) -> usize{
    let mut lind = 0;
    let mut low = boards[0].turn;
    for (ind,board) in boards.iter().skip(1).enumerate(){
        if low > board.turn{
            low = board.turn;
            lind = ind+1;
        }
    }
    lind
}

fn lst_winner(boards: &Vec<Bingo>) -> usize{
    let mut hind = 0;
    let mut high = boards[0].turn;
    for (ind,board) in boards.iter().skip(1).enumerate(){
        if high < board.turn{
            high = board.turn;
            hind = ind+1;
        }
    }
    hind
}

fn parse_numbers(input: &String) -> Vec<i32>{
    input.lines().into_iter().nth(0).unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn parse_bingo_boards(input: &String) -> Vec<Bingo>{
    let mut boards: Vec<Bingo> = Vec::new();
    for str_board in input.split("\r\n\r\n").into_iter().skip(1){
        boards.push(Bingo::new(&String::from(str_board)));
    }
    boards
}

fn play_bingo(input:String) -> (i32,i32){
    let numbers = parse_numbers(&input);
    let mut boards = parse_bingo_boards(&input);
    for board in boards.iter_mut(){
        board.win_turn(&numbers);
    }
    (boards[fst_winner(&boards)].score(&numbers),boards[lst_winner(&boards)].score(&numbers))

}

pub fn part1() {
    let input = std::fs::read_to_string("input/day4.txt").expect("ERROR");
    let (winner,_) = play_bingo(input);
    println!("Wining board -> {}", winner);
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day4.txt").expect("ERROR");
    let (_,loser) = play_bingo(input);
    println!("Losing Board -> {}",loser);
}