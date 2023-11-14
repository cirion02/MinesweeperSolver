use std::{fmt, char::from_digit, ops::Index};

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum MinesweeperCell {
    Number(usize),
    Empty,
    Question,
    Mine
}

pub fn cell_to_char(cell:MinesweeperCell) -> char{
    match cell {
        MinesweeperCell::Number(10) => 'A',
        MinesweeperCell::Number(11) => 'B',
        MinesweeperCell::Number(12) => 'C',
        MinesweeperCell::Number(val) => from_digit(val.try_into().unwrap(), 36).unwrap(),
        MinesweeperCell::Empty => ' ',
        MinesweeperCell::Question => '?',
        MinesweeperCell::Mine => 'F',
    } 
}

pub enum NextToPolicy {
    EightAround,
    XScape,
    XSmall
}

pub trait BoardIndexable {
    fn as_rows(&self) -> Vec<&[MinesweeperCell]>;

    fn get_next_to(&self, index: usize, policy: NextToPolicy) -> Vec<usize>;

    fn get_next_to_eight_around(&self, index: usize) -> Vec<usize>;

    fn get_next_to_x(&self, index: usize) -> Vec<usize>;

    fn get_next_to_x_small(&self, index: usize) -> Vec<usize>;

    fn empty_and_mine_count(&self, next_to:&Vec<usize>) -> (Vec<usize>, usize);

    fn black_white_m_minecount(&self, next_to:&Vec<usize>) -> (Vec<usize>, Vec<usize>, usize);

    fn black_white_split_minecount(&self, next_to:&Vec<usize>) -> (Vec<usize>, Vec<usize>, usize, usize);
}

#[derive(Clone)]
pub struct Board {
    pub rows : Vec<MinesweeperCell>,
    pub size : usize
}

impl BoardIndexable for Board {
    fn as_rows(&self) -> Vec<&[MinesweeperCell]> {
        self.rows.chunks(self.size).collect()
    }

    fn get_next_to(&self, index: usize, policy: NextToPolicy) -> Vec<usize>{
        match policy {
            NextToPolicy::EightAround => self.get_next_to_eight_around(index),
            NextToPolicy::XScape => self.get_next_to_x(index),
            NextToPolicy::XSmall => self.get_next_to_x_small(index)
        }
    }

    fn get_next_to_eight_around(&self, index: usize) -> Vec<usize>{
        if index >= self.size * self.size{
            panic!("index {} out of range for board", index)
        }

        let mut res = vec![];

        let up = index >= self.size;
        let down = index < self.size * (self.size-1);
        let left = index % self.size != 0;
        let right = (index+1) % self.size != 0;

        if up {
            if right {
                res.push(index - self.size + 1);
            }
            res.push(index - self.size);
            if left {
                res.push(index - self.size - 1);
            }
        }
        if right {
            res.push(index+1);
        }
        if left {
            res.push(index-1);
        }
        if down {
            if right {
                res.push(index + self.size + 1);
            }
            res.push(index + self.size);
            if left {
                res.push(index + self.size - 1);
            }
        }

        res
    }

    fn get_next_to_x(&self, index: usize) -> Vec<usize>{
        if index >= self.size * self.size{
            panic!("index {} out of range for board", index)
        }

        let mut res = vec![];

        let up = index >= self.size;
        let down = index < self.size * (self.size-1);
        let left = index % self.size != 0;
        let right = (index+1) % self.size != 0;

        if up {
            res.push(index-self.size);

            if index-self.size >= self.size {
                res.push(index-self.size-self.size);
            }
        }
        if right {
            res.push(index+1);

            if (index+2) % self.size != 0 {
                
                res.push(index+2);
            }
        }
        if left {
            res.push(index-1);

            if (index-1) % self.size != 0 {
                res.push(index-2);
            }
        }
        if down {
            res.push(index+self.size);

            if index+self.size < self.size * (self.size-1) {
                res.push(index+self.size+self.size);
            }
        }

        res
    }

    fn get_next_to_x_small(&self, index: usize) -> Vec<usize> {
        if index >= self.size * self.size{
            panic!("index {} out of range for board", index)
        }

        let mut res = vec![];

        let up = index >= self.size;
        let down = index < self.size * (self.size-1);
        let left = index % self.size != 0;
        let right = (index+1) % self.size != 0;

        if up {
            res.push(index - self.size);
        }
        if right {
            res.push(index+1);
        }
        if left {
            res.push(index-1);
        }
        if down {
            res.push(index + self.size);
        }

        res
    }

    fn empty_and_mine_count(&self, next_to:&Vec<usize>) -> (Vec<usize>, usize) {
        next_to.to_owned().into_iter().fold((Vec::new(),0), |(mut empty, mines), i| {
            match self[i] {
                MinesweeperCell::Empty => {empty.push(i); (empty, mines)},
                MinesweeperCell::Mine => (empty, mines + 1),
                _ => (empty, mines)
            }
        })
    }

    fn black_white_m_minecount(&self, next_to:&Vec<usize>) -> (Vec<usize>, Vec<usize>, usize) {
        next_to.to_owned().into_iter().fold((Vec::new(),Vec::new(),0), |(mut black, mut white, mines), i| {
            match self[i] {
                MinesweeperCell::Empty => {if is_square_id_black(i, self.size) {black.push(i)} else {white.push(i)}; (black, white, mines)},
                MinesweeperCell::Mine => (black, white, mines + if is_square_id_black(i, self.size) {1} else {2}),
                _ => (black, white, mines)
            }
        })
    }

    fn black_white_split_minecount(&self, next_to:&Vec<usize>) -> (Vec<usize>, Vec<usize>, usize, usize) {
        next_to.to_owned().into_iter().fold((Vec::new(),Vec::new(),0,0), |(mut black, mut white, black_mines, white_mines), i| {
            match self[i] {
                MinesweeperCell::Empty => {if is_square_id_black(i, self.size) {black.push(i)} else {white.push(i)}; (black, white, black_mines, white_mines)},
                MinesweeperCell::Mine => (black, white, black_mines + if is_square_id_black(i, self.size) {1} else {0}, white_mines + if is_square_id_black(i, self.size) {0} else {1}, ),
                _ => (black, white, black_mines, white_mines)
            }
        })
    }
}

fn is_square_id_black(id:usize, size:usize) -> bool{
    if size % 2 == 1 { return id % 2 == 0}

    let mut y = 0;
    let mut x = id;

    while x >= size {
        y += 1;
        x -= size;
    }

    x % 2 == y % 2
}

impl Index<usize> for Board {
    type Output = MinesweeperCell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl fmt::Display for MinesweeperCell {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            MinesweeperCell::Number(val) => write!(f, "{}", val),
            MinesweeperCell::Empty => write!(f, "Empty"),
            MinesweeperCell::Question => write!(f, "?"),
            MinesweeperCell::Mine => write!(f, "Flag"),
        }
    }
}

impl fmt::Display for Board {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.as_rows(){
            let row_text: String = row.into_iter().map(|c| cell_to_char(*c)).collect();
            write!(f, "{}\n", row_text).unwrap();
        };
        write!(f, "")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_next_to() {
        let board = Board {rows:vec![MinesweeperCell::Empty; 64], size:8};
        for i in 0..64{
            assert_eq!(board.get_next_to(i, NextToPolicy::EightAround).len(), {
                match i {
                    0|7|56|63 => 3,
                    1..=6|8|16|24|32|40|48|15|23|31|39|47|55|57..=62 => 5,
                    _ => 8
                }
            }, "testing: {}", i)
        }
    }

    #[test]
    fn correct_next_to_x() {
        let board = Board {rows:vec![MinesweeperCell::Empty; 25], size:5};
        for i in 0..25{
            println!("{}", i);
            assert_eq!(board.get_next_to(i, NextToPolicy::XScape).len(), {
                match i {
                    0|4|20|24 => 4,
                    1|3|5|9|15|19|21|23 => 5,
                    2|10|14|22 => 6,
                    6|8|16|18 => 6,
                    7|11|13|17 => 7,
                    12 => 8,
                    _ => panic!("die")
                }
            }, "testing: {} val {:?}", i, board.get_next_to(i, NextToPolicy::XScape))
        }
    }
}

pub fn cells_left(board:&Board) -> usize{
    let (empties, _) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    empties.len()
}