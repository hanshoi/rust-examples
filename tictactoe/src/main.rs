mod map;
mod utils;

fn main() {
    let mut board = map::Board::create();
    println!("Start!");
    println!("{}", board);
    for index in 0 .. 9 {
        let _input = utils::get_input();
        let mark = utils::get_tic(index);
        board.add_tic(_input, mark);
        println!("Round {}!", index + 1);
        println!("{}", board);
    }
    
}
