use std::io;

mod map;

enum Tics {
    Tic,
    Tac,
}

struct Input {
    row: usize,
    column: usize,
}


fn get_tic() -> Input {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline");
    let mut raw = guess.trim();
    if raw.len() < 2 {
        panic!("Too short input: {}", raw);
    }
    let input = Input{
        column: match &raw[0 .. 1] {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            _ => panic!("Wrong letter"),
        },
        row: match &raw[1 .. 2] {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            _ => panic!("Wrong letter"),
        },
    };
    input
}

fn main() {
    let mut board = map::create_board();
    println!("Start!");
    println!("{}", board);
    for index in 0 .. 9 {
        let _input = get_tic();
        board.rows[_input.row].row[_input.column] = match index % 2 {
            1 => 'O',
            0 => 'X',
            _ => 'O',
        };
        println!("Round {}!", index + 1);
        println!("{}", board);
    }
    
}
