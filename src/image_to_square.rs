use image_compare::rgba_hybrid_compare;
use image::RgbaImage;

use crate::board::MinesweeperCell;

fn compare_images(img1: &RgbaImage, img2: &RgbaImage) -> f64{
    let score = rgba_hybrid_compare(img1, img2).unwrap();
    score.score
}

pub struct ImgCellPair{
    img:RgbaImage,
    cell:MinesweeperCell
}

pub fn get_img_cell_pairs() -> Vec<ImgCellPair>{
    let mut res = Vec::new();

    res.push(
        ImgCellPair{img:image::open("images/empty.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Empty}
    );
    res.push(
        ImgCellPair{img:image::open("images/mine.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Mine}
    );
    res.push(
        ImgCellPair{img:image::open("images/0.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(0)}
    );
    res.push(
        ImgCellPair{img:image::open("images/1.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(1)}
    );
    res.push(
        ImgCellPair{img:image::open("images/2.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(2)}
    );
    res.push(
        ImgCellPair{img:image::open("images/3.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(3)}
    );
    res.push(
        ImgCellPair{img:image::open("images/question.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Question}
    );
    res.push(
        ImgCellPair{img:image::open("images/4.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(4)}
    );
    res.push(
        ImgCellPair{img:image::open("images/5.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(5)}
    );
    res.push(
        ImgCellPair{img:image::open("images/6.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(6)}
    );
    res.push(
        ImgCellPair{img:image::open("images/7.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(7)}
    );
    res.push(
        ImgCellPair{img:image::open("images/8.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(8)}
    );

    res
}

pub fn get_img_cell_pairs_m() -> Vec<ImgCellPair>{
    let mut res = Vec::new();

    res.push(
        ImgCellPair{img:image::open("images/empty.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Empty}
    );
    res.push(
        ImgCellPair{img:image::open("images/mine.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Mine}
    );
    res.push(
        ImgCellPair{img:image::open("images/0.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(0)}
    );
    res.push(
        ImgCellPair{img:image::open("images/1.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(1)}
    );
    res.push(
        ImgCellPair{img:image::open("images/2.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(2)}
    );
    res.push(
        ImgCellPair{img:image::open("images/3.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(3)}
    );
    res.push(
        ImgCellPair{img:image::open("images/question.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Question}
    );
    res.push(
        ImgCellPair{img:image::open("images/4.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(4)}
    );
    res.push(
        ImgCellPair{img:image::open("images/5.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(5)}
    );
    res.push(
        ImgCellPair{img:image::open("images/6.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(6)}
    );
    res.push(
        ImgCellPair{img:image::open("images/7.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(7)}
    );
    res.push(
        ImgCellPair{img:image::open("images/8.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(8)}
    );
    res.push(
        ImgCellPair{img:image::open("images/large_numbers/9.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(9)}
    );
    res.push(
        ImgCellPair{img:image::open("images/large_numbers/10.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(10)}
    );
    res.push(
        ImgCellPair{img:image::open("images/large_numbers/11.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(11)}
    );

    res.push(
        ImgCellPair{img:image::open("images/white/white_empty.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Empty}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_mine.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Mine}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_0.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(0)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_1.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(1)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_2.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(2)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_3.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(3)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_question.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Question}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_4.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(4)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_5.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(5)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_6.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(6)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_7.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(7)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_8.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(8)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_9.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(9)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_10.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(10)}
    );
    res.push(
        ImgCellPair{img:image::open("images/white/white_11.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(11)}
    );

    res
}

pub fn get_img_cell_pairs_l() -> Vec<ImgCellPair>{
    let mut res = Vec::new();

    res.push(
        ImgCellPair{img:image::open("images/empty.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Empty}
    );
    res.push(
        ImgCellPair{img:image::open("images/mine.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Mine}
    );
    res.push(
        ImgCellPair{img:image::open("images/0.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(0)}
    );
    res.push(
        ImgCellPair{img:image::open("images/1.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(1)}
    );
    res.push(
        ImgCellPair{img:image::open("images/2.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(2)}
    );
    res.push(
        ImgCellPair{img:image::open("images/3.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(3)}
    );
    res.push(
        ImgCellPair{img:image::open("images/question.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Question}
    );
    res.push(
        ImgCellPair{img:image::open("images/4.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(4)}
    );
    res.push(
        ImgCellPair{img:image::open("images/5.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(5)}
    );
    res.push(
        ImgCellPair{img:image::open("images/6.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(6)}
    );
    res.push(
        ImgCellPair{img:image::open("images/7.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(7)}
    );
    res.push(
        ImgCellPair{img:image::open("images/8.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(8)}
    );
    res.push(
        ImgCellPair{img:image::open("images/large_numbers/9.jpg").unwrap().into_rgba8(), cell:MinesweeperCell::Number(9)}
    );

    res
}

pub fn parse_cell(rgba_img:RgbaImage, img_cell_pairs:&Vec<ImgCellPair>) -> MinesweeperCell{
    for pair in img_cell_pairs{
        let score = compare_images(&rgba_img, &pair.img);
        if score > 0.93{
            return pair.cell.clone();
        }
    }

    //println!("Trouble finding match");

    let mut highest_score:f64 = -1.0;
    let mut highest_cell = MinesweeperCell::Empty;

    for pair in img_cell_pairs{
        let score = compare_images(&rgba_img, &pair.img);
        if score > highest_score{
            highest_score = score;
            highest_cell = pair.cell;
        }
    }

    //println!("Matched: {} at score {}", highest_cell, highest_score);

    highest_cell.clone()
}