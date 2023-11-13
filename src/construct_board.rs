use crate::board_image_getter::{get_whole_window, get_square_image};
use crate::image_to_square::{parse_cell, get_img_cell_pairs, get_img_cell_pairs_m, get_img_cell_pairs_l, get_img_cell_pairs_n};
use crate::board::{MinesweeperCell, Board};

pub enum VisionType {
    Normal,
    ChessBig,
    UpToNine,
    ChessSmall,
}

pub fn get_board(board_size:usize, vision_type:VisionType) -> Board{

    let img = get_whole_window();

    let mut board: Vec<MinesweeperCell> = Vec::new();

    let mut x=0;
    let mut y=0;

    let board_size_u32 = board_size.try_into().unwrap();

    
    let img_cell_pairs = match vision_type {
        VisionType::Normal => get_img_cell_pairs(),
        VisionType::ChessBig => get_img_cell_pairs_m(),
        VisionType::UpToNine => get_img_cell_pairs_l(),
        VisionType::ChessSmall => get_img_cell_pairs_n(),
    };

    while y < board_size_u32{
        board.push(parse_cell(get_square_image(&img, board_size, x, y), &img_cell_pairs));

        x += 1;

        if x>=board_size_u32{
            x = 0;
            y += 1;
        }
    }


    Board {rows:board, size:board_size.into()}
}