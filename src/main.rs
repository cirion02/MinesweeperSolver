use algorithms::Solver;

mod board_image_getter;
mod image_to_square;
mod board;
mod construct_board;
mod algorithms;
mod game_inputs;
use std::{thread, time};

fn main() {
    let board_size = 8;
    let mut i=0;
    while i<100{

        let board = construct_board::get_board(board_size);

        println!("{}", board);

        let solver = algorithms::vanilla_solver();

        let res = solver.get_known_squares(&board);

        algorithms::display_known_squares(&res, board_size);

        if !game_inputs::click_known_cells(res, board_size) {break};

        thread::sleep(time::Duration::from_millis(500));

        i += 1;
    }
}