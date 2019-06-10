use std::io;

pub struct Input {
    pub row: usize,
    pub column: usize,
}


pub fn get_input() -> Input {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline");
    let raw = guess.trim();
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

pub fn get_tic(turn: u8) -> char {
    let mark = match turn % 2 {
        1 => 'O',
        0 => 'X',
        _ => 'O',
    };
    mark
}
