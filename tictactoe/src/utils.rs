use std::io;

pub struct Input {
    pub row: usize,
    pub column: usize,
}

impl Input {
    fn create(col: char, row: char) -> Input {
      Input{
        column: match col {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => panic!("Wrong letter"),
        },
        row: match row {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            _ => panic!("Wrong letter"),
        },
      }  
    }
}

fn read_char_input() -> (char, char) {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline");
    let raw = guess.trim();
    if raw.len() < 2 {
        panic!("Too short input: {}", raw);
    }
    let mut raw_chars = raw.chars();
    (
        raw_chars.next().unwrap(),
        raw_chars.next().unwrap()
    )
}

pub fn get_input() -> Input {
    let (col_char, row_char) = read_char_input();
    Input::create(col_char, row_char)
}

pub fn get_tic(turn: u8) -> char {
    match turn % 2 {
        1 => 'O',
        0 => 'X',
        _ => 'O',
    }
}
