use algorithms::Solver;

mod board_image_getter;
mod image_to_square;
mod board;
mod construct_board;
mod algorithms;
mod game_inputs;
mod linear_programming_solvers;
use std::{thread, time};

fn main() {

    /*
    let board_size = 8;

    let img = board_image_getter::get_square_image(&board_image_getter::get_whole_window(), 8, 4, 3);
    board_image_getter::save_image(img);
    
    */
    let board_size = 5;

    let mut j = 0;

    while j < 1 {

        let mut i=0;

        solve_l(board_size);

        game_inputs::click_next_puzzle();
        thread::sleep(time::Duration::from_millis(500));
        j += 1;
    }

    /*
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
    */
}

fn solve_v(board_size:usize){
    let mut i=0;
    while i<100{

        let board = construct_board::get_board(board_size, construct_board::VisionType::Normal);

        println!("{}", board);

        let res = linear_programming_solvers::find_known_squares(&board, linear_programming_solvers::create_constraint_set_v);

        algorithms::display_known_squares(&res, board_size);

        if !game_inputs::click_known_cells(&res, board_size) {panic!("I cannot solve this puzzle")};

        if board::cells_left(&board) == res.mines.len() + res.safe.len() {
            break;
        }

        thread::sleep(time::Duration::from_millis(100));

        i += 1;
    }
}

fn solve_q(board_size:usize){
    let mut i=0;
    while i<100{

        let board = construct_board::get_board(board_size, construct_board::VisionType::Normal);

        println!("{}", board);

        let res = linear_programming_solvers::find_known_squares(&board, linear_programming_solvers::create_constraint_set_q);

        algorithms::display_known_squares(&res, board_size);

        if !game_inputs::click_known_cells(&res, board_size) {panic!("I cannot solve this puzzle")};

        if board::cells_left(&board) == res.mines.len() + res.safe.len() {
            break;
        }

        thread::sleep(time::Duration::from_millis(100));

        i += 1;
    }
}

fn solve_m(board_size:usize){
    let mut i=0;
    while i<100{

        let board = construct_board::get_board(board_size, construct_board::VisionType::ChessBig);

        println!("{}", board);

        let res = linear_programming_solvers::find_known_squares(&board, linear_programming_solvers::create_constraint_set_m);

        algorithms::display_known_squares(&res, board_size);

        if !game_inputs::click_known_cells(&res, board_size) {panic!("I cannot solve this puzzle")};

        if board::cells_left(&board) == res.mines.len() + res.safe.len() {
            break;
        }

        thread::sleep(time::Duration::from_millis(100));

        i += 1;
    }
}

fn solve_l(board_size:usize){
    let mut i=0;
    while i<100{

        let board = construct_board::get_board(board_size, construct_board::VisionType::UpToNine);

        println!("{}", board);

        let res = linear_programming_solvers::find_known_squares(&board, linear_programming_solvers::create_constraint_set_l);

        algorithms::display_known_squares(&res, board_size);

        if !game_inputs::click_known_cells(&res, board_size) {panic!("I cannot solve this puzzle")};

        if board::cells_left(&board) == res.mines.len() + res.safe.len() {
            break;
        }

        thread::sleep(time::Duration::from_millis(100));

        i += 1;
    }
}