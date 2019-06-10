use std::fmt;

use utils::Input;


#[derive(Copy, Clone)]
pub struct Row {
    pub row: [char; 3],
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tictac in self.row.iter() {
            write!(f, "| {} ", tictac)?;
        }
        write!(f, "|")
    }
}


pub struct Board {
    pub rows: [Row; 3],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  | a | b | c |\n")?;
        write!(f, "---------------\n")?;
        for (index, row) in self.rows.iter().enumerate() {
            write!(f, "{} {}\n", index + 1, row)?;
        }
        write!(f, "---------------\n")
    } 
}


impl Board {
    pub fn create() -> Board {
        // fill up board
        Board{
            rows: [Row{
                row: [' '; 3]
            }; 3]
        }
    }

    pub fn add_tic(&mut self, input: Input, mark: char) {
        self.rows[input.row].row[input.column] = mark;
    }
}
