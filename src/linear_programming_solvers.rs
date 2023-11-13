use crate::board::{Board, BoardIndexable, MinesweeperCell, NextToPolicy};
use crate::algorithms::KnownSquares;

use highs::{Sense, HighsModelStatus, RowProblem};
use std::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
enum ConstraitType {
    Equality,
    Minimum,
    BlackWhiteEquality,
    OffByOne,
    DifferenceOfColors(f64),
}

#[derive(Clone)]
struct Constraint {
    constrait_type : ConstraitType,
    value: usize,
    cells : Vec<usize>,
    cells2 : Vec<usize>,
}

#[derive(Clone)]
pub struct ConstraintSet {
    cells : Vec<usize>,
    constraints : Vec<Constraint>,
    boardsize : usize
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
enum ProbeResult {
    Mine,
    Safe,
    Unknown
}

impl fmt::Display for ConstraitType {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConstraitType::Equality => write!(f, "=="),
            ConstraitType::Minimum => write!(f, ">="),
            ConstraitType::BlackWhiteEquality => write!(f, "M="),
            ConstraitType::OffByOne => write!(f, "L="),
            ConstraitType::DifferenceOfColors(_) => write!(f, "N="),
        }
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}: {:?} + {:?}", self.constrait_type, self.value, self.cells, self.cells2)
    }
}

impl fmt::Display for ConstraintSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.constraints.clone().into_iter() {
            writeln!(f, "{}", x).unwrap();
        };
        write!(f, "")
    }
}

fn combine_constraint_sets(set1:ConstraintSet, set2:ConstraintSet) -> ConstraintSet {
    ConstraintSet { 
        cells: set1.cells, 
        constraints: {
            let mut v = set1.constraints;
            v.extend(set2.constraints);
            v
        }, 
        boardsize: set1.boardsize 
    }
}

fn create_constraints_for_cell_v(id:usize, board:&Board) -> Vec<Constraint> {
    match board[id] {
        MinesweeperCell::Number(x) => {
            let next_to = board.get_next_to(id, NextToPolicy::EightAround);
            let (empty, bombs) = board.empty_and_mine_count(&next_to);
            if empty.len() == 0 { return vec![] };
            return vec![Constraint { constrait_type: ConstraitType::Equality, value: x - bombs, cells: empty, cells2: vec![] }];
        },
        _ => return vec![]
    }
}

fn create_constraint_set_normal_mines(board:&Board) -> ConstraintSet{
    
    let (empty_cells, _) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    ConstraintSet {
        constraints:
            (0..board.size*board.size).fold(vec![], 
                |mut vec, id| 
                {
                    vec.extend(create_constraints_for_cell_v(id, board));
                    vec
                }),
        boardsize: board.size,
        cells:empty_cells
    }
}

fn create_constraint_set_minecount(board:&Board) -> ConstraintSet{
    let total_count:usize = match board.size{
        5 => 10,
        6 => 14,
        7 => 20,
        8 => 26,
        _ => panic!("Bad board size")
    };

    let (empty_cells, mines_placed) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    let mut constraints = vec![Constraint {constrait_type: ConstraitType::Equality, value: total_count-mines_placed, cells: empty_cells.clone(), cells2: vec![]}];

    ConstraintSet {
        constraints:constraints,
        boardsize: board.size,
        cells:empty_cells.clone()
    }
}

fn create_q_added_constraint_set(board:&Board) -> ConstraintSet{
    let (empty_cells, mines_placed) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    let mut constraints = vec![];

    for x in 0..board.size - 1{
        for y in 0..board.size - 1{
            let cells = vec![x + y * board.size, x+1 + y * board.size, x + (y+1) * board.size, x+1 + (y+1) * board.size];
            let (empty, mines) = board.empty_and_mine_count(&cells);
            if mines == 0 {
                constraints.push(
                    Constraint{
                        constrait_type: ConstraitType::Minimum,
                        value: 1,
                        cells: empty,
                        cells2: vec![]
                    }
                )
            }
        }
    }

    ConstraintSet {
        constraints:constraints,
        boardsize: board.size,
        cells:empty_cells
    } 
}

fn create_constraints_for_cell_m(id:usize, board:&Board) -> Vec<Constraint> {
    match board[id] {
        MinesweeperCell::Number(x) => {
            let next_to = board.get_next_to(id, NextToPolicy::EightAround);
            let (black, white, bombs) = board.black_white_m_minecount(&next_to);
            if black.len() + white.len() == 0 { return vec![] };
            return vec![Constraint { constrait_type: ConstraitType::BlackWhiteEquality, value: x - bombs, cells: black, cells2: white }];
        },
        _ => return vec![]
    }
}

fn create_constraint_set_multiple_mines(board:&Board) -> ConstraintSet{
    
    let (empty_cells, _) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    ConstraintSet {
        constraints:
            (0..board.size*board.size).fold(vec![], 
                |mut vec, id| 
                {
                    vec.extend(create_constraints_for_cell_m(id, board));
                    vec
                }),
        boardsize: board.size,
        cells:empty_cells
    }
}

fn create_constraints_for_cell_l(id:usize, board:&Board) -> Vec<Constraint> {
    match board[id] {
        MinesweeperCell::Number(x) => {
            let next_to = board.get_next_to(id, NextToPolicy::EightAround);
            let (empty, bombs) = board.empty_and_mine_count(&next_to);
            if empty.len() == 0 { return vec![] };
            if x == 0 {
                return vec![Constraint { constrait_type: ConstraitType::Equality, value: 1 - bombs, cells: empty, cells2: vec![] }]
            }
            if bombs > x {
                return vec![Constraint { constrait_type: ConstraitType::Equality, value: 0, cells: empty, cells2: vec![] }]
            }
            return vec![Constraint { constrait_type: ConstraitType::OffByOne, value: x - bombs, cells: empty, cells2: vec![] }];
        },
        _ => return vec![]
    }
}

fn create_constraint_set_liar_mines(board:&Board) -> ConstraintSet{
    
    let (empty_cells, _) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    ConstraintSet {
        constraints:
            (0..board.size*board.size).fold(vec![], 
                |mut vec, id| 
                {
                    vec.extend(create_constraints_for_cell_l(id, board));
                    vec
                }),
        boardsize: board.size,
        cells:empty_cells
    }
}


fn create_constraint_set_minecount_b(board:&Board) -> ConstraintSet{
    let total_count:usize = match board.size{
        5 => 10,
        6 => 12,
        7 => 21,
        8 => 24,
        _ => panic!("Bad board size")
    };

    let (empty_cells, mines_placed) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    let mut constraints = vec![Constraint {constrait_type: ConstraitType::Equality, value: total_count-mines_placed, cells: empty_cells.clone(), cells2: vec![]}];

    ConstraintSet {
        constraints:constraints,
        boardsize: board.size,
        cells:empty_cells.clone()
    }
}

fn create_b_added_constraint_set(board:&Board) -> ConstraintSet{
    let (empty_cells, mines_placed) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    let mines_per_row = match board.size {
        5 => 2,
        6 => 2,
        7 => 3,
        8 => 3,
        _ => panic!("Bad board size")
    };

    let mut constraints = vec![];

    for i in 0..board.size{
        //Rows
        let (empty_cells, mines_placed) = board.empty_and_mine_count(&(0..board.size).map(|x|x+i*board.size).collect());

        constraints.push(Constraint { constrait_type: ConstraitType::Equality, value: mines_per_row - mines_placed, cells: empty_cells, cells2: vec![] });

        //Columns
        let (empty_cells, mines_placed) = board.empty_and_mine_count(&(0..board.size).map(|x|(x*board.size)+i).collect());

        constraints.push(Constraint { constrait_type: ConstraitType::Equality, value: mines_per_row - mines_placed, cells: empty_cells, cells2: vec![] });
    }

    ConstraintSet {
        constraints:constraints,
        boardsize: board.size,
        cells:empty_cells
    } 
}

fn create_constraints_for_cell_n(id:usize, board:&Board) -> Vec<Constraint> {
    match board[id] {
        MinesweeperCell::Number(x) => {
            let next_to = board.get_next_to(id, NextToPolicy::EightAround);
            let (black, white, black_bombs, white_bomds) = board.black_white_split_minecount(&next_to);
            if black.len() + white.len() == 0 { return vec![] };
            return vec![Constraint { constrait_type: ConstraitType::DifferenceOfColors(black_bombs as f64-white_bomds as f64), value: x, cells: black, cells2: white }];
        },
        _ => return vec![]
    }
}

fn create_constraint_set_negation_mines(board:&Board) -> ConstraintSet{
    
    let (empty_cells, _) = board.empty_and_mine_count(&(0..board.size*board.size).collect());

    ConstraintSet {
        constraints:
            (0..board.size*board.size).fold(vec![], 
                |mut vec, id| 
                {
                    vec.extend(create_constraints_for_cell_n(id, board));
                    vec
                }),
        boardsize: board.size,
        cells:empty_cells
    }
}

pub fn create_constraint_set_v(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_minecount(board), create_constraint_set_normal_mines(board))
}

pub fn create_constraint_set_q(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_v(board), create_q_added_constraint_set(board))
}

pub fn create_constraint_set_m(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_minecount(board), create_constraint_set_multiple_mines(board))
}

pub fn create_constraint_set_l(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_minecount(board), create_constraint_set_liar_mines(board))
}

pub fn create_constraint_set_b(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_normal_mines(board), combine_constraint_sets(create_constraint_set_minecount_b(board), create_b_added_constraint_set(board)))
}

pub fn create_constraint_set_n(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_minecount(board), create_constraint_set_negation_mines(board))
}

fn probe_cell(constraints:&ConstraintSet, probe_id:usize) -> ProbeResult{
    let mut lookup: HashMap<usize, usize> = HashMap::new();
    for i in 0..constraints.cells.len() {
        lookup.insert( constraints.cells[i], i);
    }

    let mut pb = RowProblem::default();

    let colums: Vec<_> = (0..constraints.cells.len()).map(
        |v| pb.add_integer_column(if constraints.cells[v] == probe_id {1.} else {0.}, 0..1)
    ).collect();

    for constraint in &constraints.constraints {
        let value:f64 = constraint.value as f64;
        match constraint.constrait_type {
            ConstraitType::Equality => {
                let cells : Vec<(_, f64)> = constraint.cells.clone().into_iter().map(|id|(colums[*lookup.get(&id).unwrap()], 1.)).collect();
                pb.add_row(value..=value, cells)
            },
            ConstraitType::Minimum => {
                let cells : Vec<(_, f64)> = constraint.cells.clone().into_iter().map(|id|(colums[*lookup.get(&id).unwrap()], 1.)).collect();
                pb.add_row(value.., cells)
            },
            ConstraitType::BlackWhiteEquality => {
                let mut cells : Vec<(_, f64)> = constraint.cells.clone().into_iter().map(|id|(colums[*lookup.get(&id).unwrap()], 1.)).collect();
                cells.extend(constraint.cells2.clone().into_iter().map(|id|(colums[*lookup.get(&id).unwrap()], 2.)));
                pb.add_row(value..=value, cells)
            },
            ConstraitType::OffByOne => {
                let liar_cell = pb.add_integer_column(0., 0..1);
                let mut cells : Vec<(_, f64)> = constraint.cells.clone().into_iter().map(|id|(colums[*lookup.get(&id).unwrap()], 1.)).collect();
                cells.push((liar_cell, 2.));
                pb.add_row((value+1.0)..=(value+1.0), cells)
            }
            ConstraitType::DifferenceOfColors(black_bias) => {
                let abs_cell = pb.add_integer_column(0., 0..1);
                let mut cells : Vec<(_, f64)> = constraint.cells.clone().into_iter().map(|id|(colums[*lookup.get(&id).unwrap()], 1.)).collect();
                cells.extend(constraint.cells2.clone().into_iter().map(|id|(colums[*lookup.get(&id).unwrap()], -1.)));
                cells.push((abs_cell, value*2.0));
                pb.add_row((value-black_bias)..=(value-black_bias), cells)
            }
        }
    }

    let max = pb.clone().optimise(Sense::Maximise).solve();

    assert_eq!(max.status(), HighsModelStatus::Optimal);

    let score = max.get_solution().columns()[*lookup.get(&probe_id).unwrap()];

    if score < 0.5 {
        return ProbeResult::Safe;
    }

    let min = pb.optimise(Sense::Minimise).solve();

    assert_eq!(min.status(), HighsModelStatus::Optimal);

    let score2 = min.get_solution().columns()[*lookup.get(&probe_id).unwrap()];

    if score2 > 0.5 {
        return ProbeResult::Mine;
    }

    return ProbeResult::Unknown;
}

pub fn find_known_squares(board:&Board, constaints_building:fn(&Board) -> ConstraintSet) -> KnownSquares{
    let s = constaints_building(&board);

    println!("{}", s);

    let mut saves = HashSet::new();
    let mut mines = HashSet::new();


    for i in 0..board.size * board.size{
        if board[i] == MinesweeperCell::Empty {
            match probe_cell(&s, i) {
                ProbeResult::Mine => mines.insert(i),
                ProbeResult::Safe => saves.insert(i),
                ProbeResult::Unknown => true
            };
        }
    }

    return KnownSquares { mines: mines, safe: saves }
}