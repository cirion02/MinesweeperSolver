use image::{RgbaImage, GenericImageView};
use image::imageops::{resize, Gaussian};
use win_screenshot::prelude::*;

const EIGHT_TOP_LEFT: (u32,u32) = (606,271);
const SEVEN_TOP_LEFT: (u32, u32) = (652,316);
const SQUARE_SIZE: (u32,u32) = (92,92);
const SQUARE_IMAGE_CROP_SIZE: (u32,u32) = (83,83);
const DOWNSCALE_SIZE: (u32,u32) = (10,10);


pub fn get_whole_window() -> RgbaImage{
    // Capture window if you know the exact name
    let hwnd = find_window("Minesweeper Variants").unwrap();
    let buf = capture_window(hwnd).unwrap();

    // convert to image and save
    RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap()
}


pub fn save_image(img:RgbaImage){
    img.save("images/screenshot.jpg").unwrap();
}


pub fn get_square_image(full_screenshot:&RgbaImage, board_size:usize, square_x:u32, square_y:u32) -> RgbaImage {
    let x = match board_size {
        8 => EIGHT_TOP_LEFT.0,
        7 => SEVEN_TOP_LEFT.0,
        6 => EIGHT_TOP_LEFT.0 + SQUARE_SIZE.0,
        5 => SEVEN_TOP_LEFT.0 + SQUARE_SIZE.0,
        _ => panic!("Board size is not 5-8")
    } + square_x * SQUARE_SIZE.0;

    let y = match board_size {
        8 => EIGHT_TOP_LEFT.1,
        7 => SEVEN_TOP_LEFT.1,
        6 => EIGHT_TOP_LEFT.1 + SQUARE_SIZE.1,
        5 => SEVEN_TOP_LEFT.1 + SQUARE_SIZE.1,
        _ => panic!("Board size is not 5-8")
    } + square_y * SQUARE_SIZE.1;


    downscale_image(full_screenshot.view(x,y,SQUARE_IMAGE_CROP_SIZE.0,SQUARE_IMAGE_CROP_SIZE.1).to_image())
}

fn downscale_image(img: RgbaImage) -> RgbaImage{
    resize(&img, DOWNSCALE_SIZE.0, DOWNSCALE_SIZE.1, Gaussian)
}