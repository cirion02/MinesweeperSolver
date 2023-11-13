use mouse_rs::{Mouse,types::keys::Keys};
use crate::algorithms::KnownSquares;
use std::{thread, time};

const EIGHT_TOP_LEFT: (i32,i32) = (640,309);
const SEVEN_TOP_LEFT: (i32,i32) = (685,354);
const SQUARE_SIZE: (i32, i32) = (88,88);

fn click_square(square_x:usize, square_y:usize, board_size:usize, flag:bool){
    if square_x >= board_size {
        panic!("Invalid X coordinate {} on board of size {}", square_x, board_size);
    }

    if square_y >= board_size {
        panic!("Invalid Y coordinate {} on board of size {}", square_y, board_size);
    }

    let x = match board_size {
        8 => EIGHT_TOP_LEFT.0,
        7 => SEVEN_TOP_LEFT.0,
        6 => EIGHT_TOP_LEFT.0 + SQUARE_SIZE.0,
        5 => SEVEN_TOP_LEFT.0 + SQUARE_SIZE.0,
        _ => panic!("Board size is not 5-8")
    } + i32::try_from(square_x).unwrap() * SQUARE_SIZE.0;

    let y = match board_size {
        8 => EIGHT_TOP_LEFT.1,
        7 => SEVEN_TOP_LEFT.1,
        6 => EIGHT_TOP_LEFT.1 + SQUARE_SIZE.1,
        5 => SEVEN_TOP_LEFT.1 + SQUARE_SIZE.1,
        _ => panic!("Board size is not 5-8")
    } + i32::try_from(square_y).unwrap() * SQUARE_SIZE.1;

    let mouse = Mouse::new();
    mouse.move_to(x, y).expect("Unable to move mouse");
    let delay = time::Duration::from_millis(20);
    thread::sleep(delay);
    mouse.press(if flag {&Keys::RIGHT} else {&Keys::LEFT}).expect("Unable to press button");
    thread::sleep(delay);
    mouse.release(if flag {&Keys::RIGHT} else {&Keys::LEFT}).expect("Unable to release button");
    thread::sleep(delay);
}

pub fn click_known_cells(known_cells:&KnownSquares, board_size:usize) -> bool{
    let mut actions = false;
    for cell in &known_cells.safe{
        click_square(cell % board_size, cell / board_size, board_size, false);
        actions = true;
    }
    for cell in &known_cells.mines{
        click_square(cell % board_size, cell / board_size, board_size, true);
        actions = true;
    }
    let mouse = Mouse::new();
    let ten_millis = time::Duration::from_millis(20);
    thread::sleep(ten_millis);
    mouse.move_to(50, 250).expect("Unable to move mouse");
    actions
}

pub fn click_next_puzzle(){
    let mouse = Mouse::new();
    mouse.move_to(1039, 858).expect("Unable to move mouse");
    let delay = time::Duration::from_millis(25);
    thread::sleep(delay);
    mouse.press(&Keys::LEFT).expect("Unable to press button");
    thread::sleep(delay);
    mouse.release(&Keys::LEFT).expect("Unable to release button");
    thread::sleep(delay);
    mouse.move_to(50, 250).expect("Unable to move mouse");
}