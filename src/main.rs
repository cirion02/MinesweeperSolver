use algorithms::Solver;
use construct_board::VisionType;

mod board_image_getter;
mod image_to_square;
mod board;
mod construct_board;
mod algorithms;
mod game_inputs;
mod linear_programming_solvers;
mod plus_linear_solvers;
use std::{thread, time};

fn main() {

    /*
    let board_size = 8;

    let img = board_image_getter::get_square_image(&board_image_getter::get_whole_window(), 8, 4, 3);
    board_image_getter::save_image(img);
    
    */
    let board_size = 8;

    let mut j = 0;

    while j < 10 {

        let mut i=0;

        //solve_generic(board_size, VisionType::ChessSmall, plus_linear_solvers::create_constraint_set_tn);

        solve_b(board_size);

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
    solve_generic(board_size, VisionType::Normal, linear_programming_solvers::create_constraint_set_v)
}

fn solve_q(board_size:usize){
    solve_generic(board_size, VisionType::Normal, linear_programming_solvers::create_constraint_set_q)
}

fn solve_m(board_size:usize){
    solve_generic(board_size, VisionType::ChessBig, linear_programming_solvers::create_constraint_set_m)
}

fn solve_l(board_size:usize){
    solve_generic(board_size, VisionType::UpToNine, linear_programming_solvers::create_constraint_set_l)
}

fn solve_b(board_size:usize){
    solve_generic(board_size, VisionType::Normal, linear_programming_solvers::create_constraint_set_b)
}

fn solve_n(board_size:usize){
    solve_generic(board_size, VisionType::ChessSmall, linear_programming_solvers::create_constraint_set_n)
}

fn solve_x(board_size:usize){
    solve_generic(board_size, VisionType::Normal, linear_programming_solvers::create_constraint_set_x)
}

fn solve_t(board_size:usize){
    solve_generic(board_size, VisionType::Normal, linear_programming_solvers::create_constraint_set_t)
}

fn solve_x_prime(board_size:usize){
    solve_generic(board_size, VisionType::UpToFour, linear_programming_solvers::create_constraint_set_x_prime)
}

fn solve_generic(board_size:usize, vision_type:VisionType, constrain_set_maker:fn(&board::Board) -> linear_programming_solvers::ConstraintSet){
    let mut i=0;
    while i<100{

        let board = construct_board::get_board(board_size, vision_type);

        println!("{}", board);

        let res = linear_programming_solvers::find_known_squares(&board, constrain_set_maker);

        algorithms::display_known_squares(&res, board_size);

        if !game_inputs::click_known_cells(&res, board_size) {panic!("I cannot solve this puzzle")};

        if board::cells_left(&board) == res.mines.len() + res.safe.len() {
            break;
        }

        thread::sleep(time::Duration::from_millis(70));

        i += 1;
    }
}