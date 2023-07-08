use std::collections::HashSet;

use crate::board::{Board, BoardIndexable, NextToPolicy, MinesweeperCell};
use itertools::Itertools;

pub struct KnownSquares {
    pub mines:HashSet<usize>,
    pub safe:HashSet<usize>
}

pub fn display_known_squares(known_squares:&KnownSquares, board_size:usize){
    let mut spaces = vec!['_'; board_size*board_size];

    for mine in known_squares.mines.to_owned() {
        spaces[mine] = 'X';
    }
    for mine in known_squares.safe.to_owned() {
        spaces[mine] = 'O';
    }

    for row in spaces.chunks(board_size){
        println!("{}", String::from_iter(row.iter()));
    }
}

pub trait Solver {
    fn get_known_squares(self, board:&Board) -> KnownSquares;
}

trait ZoneData {
    fn min(&self) -> usize;
    fn max(&self) -> usize;
    fn count(&self) -> usize;

    fn sections(&self, other:&Self) -> (Vec<usize>, Vec<usize>, Vec<usize>);
    fn has_overlap(&self, other:&Self) -> bool;
}

#[derive(Clone)]
#[derive(Debug)]
struct Zone {
    cells:Vec<usize>,
    minecount:Vec<usize>
}

impl ZoneData for Zone {
    fn min(&self) -> usize{
        *self.minecount.iter().min().unwrap()
    }
    fn max(&self) -> usize{
        *self.minecount.iter().max().unwrap()
    }
    fn count(&self) -> usize {
        self.cells.len()
    }

    fn sections(&self, other:&Self) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
        let first_set: HashSet<usize> = self.cells.to_owned().into_iter().collect();
        let second_set: HashSet<usize> = other.cells.to_owned().into_iter().collect();

        (   
            first_set.difference(&second_set).cloned().collect(),
            first_set.intersection(&second_set).cloned().collect(),
            second_set.difference(&first_set).cloned().collect()
        )
    }

    fn has_overlap(&self, other:&Self) -> bool {
        for elem in self.cells.to_owned() {
            if other.cells.contains(&elem) {
                return true;
            }
        }
        false
    }
}

type ZoneRule = fn(&Board) -> Vec<Zone>;

type ZoneCombineRule = fn(&Zone, &Zone) -> Option<Zone>;

type SolveRule = fn(&Zone) -> Option<KnownSquares>;

type SolveRuleTwo = fn(&Zone, &Zone) -> Option<KnownSquares>;

pub struct ZoneRuleSolver {
    zone_rules:Vec<ZoneRule>,
    zone_combine_rules:Vec<ZoneCombineRule>,
    solve_rules:Vec<SolveRule>,
    solve_rules_two:Vec<SolveRuleTwo>,
}

impl Solver for ZoneRuleSolver {
    fn get_known_squares(self, board:&Board) -> KnownSquares {
        let zones = self.zone_rules.into_iter().flat_map(|z| z(board));

        let temp = zones.clone().combinations(2).flat_map(|zs| self.zone_combine_rules.to_owned().into_iter().filter_map(move |r| if zs[0].has_overlap(&zs[1]) {r(&zs[0],&zs[1])} else {None}));

        /* TODO: clean zones between steps */

        let new_zones = zones.chain(temp);

        
        //let temp: Vec<Zone> = new_zones.clone().collect();
        //println!("{:?}", temp);

        let known_squares_one = new_zones.clone().flat_map(|z| self.solve_rules.to_owned().into_iter().filter_map(move |r| r(&z)));

        let known_squares_two = new_zones.combinations(2).flat_map(|zs| self.solve_rules_two.to_owned().into_iter().filter_map(move |r| if zs[0].has_overlap(&zs[1]) {r(&zs[0],&zs[1])} else {None}));

        known_squares_one.chain(known_squares_two).fold(KnownSquares {mines: HashSet::new(), safe:HashSet::new()},
        |mut acc, new| {
            acc.mines.extend(&new.mines);
            acc.safe.extend(&new.safe);
            acc
            }
        )
    }
}

// <ZoneRules>

type CellZoneRule = fn(index: usize, board: &Board) -> Option<Zone>;

fn apply_to_all_squares(board: &Board, rule:CellZoneRule) -> Vec<Zone>{
    (0..board.size*board.size).filter_map(|i| rule(i, board)).collect()
}

fn add_zone_for_number_v(board: &Board) -> Vec<Zone>{
    fn check_index(index: usize, board: &Board) -> Option<Zone>{
        let x = match board[index] {
            MinesweeperCell::Number(i) => i,
            _ => return None
        };

        let next_to = board.get_next_to(index, NextToPolicy::EightAround);
        let (empty, bomb) = board.empty_and_mine_count(&next_to);
        if empty.len() == 0 {return None};

        Some(Zone {cells:empty, minecount:vec![x-bomb]})
    }
    apply_to_all_squares(board, check_index)
}

fn add_zone_for_number_x(board: &Board) -> Vec<Zone>{
    fn check_index(index: usize, board: &Board) -> Option<Zone>{
        let x = match board[index] {
            MinesweeperCell::Number(i) => i,
            _ => return None
        };

        let next_to = board.get_next_to(index, NextToPolicy::XScape);
        let (empty, bomb) = board.empty_and_mine_count(&next_to);
        if empty.len() == 0 {return None};

        Some(Zone {cells:empty, minecount:vec![x-bomb]})
    }
    apply_to_all_squares(board, check_index)
}

fn total_mines_standard(board: &Board) -> Vec<Zone>{
    let total_count = match board.size{
        5 => 10,
        6 => 14,
        7 => 20,
        8 => 26,
        _ => panic!("Bad board size")
    };
    let (cells, mines) = board.empty_and_mine_count(&(0..board.size * board.size).collect_vec());
    vec![
        Zone {cells:cells, minecount:vec![total_count-mines]}
    ]
}

fn add_q_spaces(board: &Board) -> Vec<Zone>{
    fn check_block(cells:Vec<usize>, board:&Board) -> Option<Vec<usize>>{
        let (new_cells, mines) = board.empty_and_mine_count(&cells);

        if mines > 0 {return None}

        Some(new_cells)
    }

    let count = board.size - 1;

    (0..count).flat_map(|x|(0..count).filter_map(move |y| check_block(vec!
        [x+y*board.size,
         x+y*board.size+1,
         x+(y+1)*board.size,
         x+(y+1)*board.size+1], 
        board)).map(|cells| Zone{cells, minecount:vec![1,2,3,4]})).collect_vec()
}

fn add_connected_mines(board:&Board) -> Vec<Zone>{
    fn check_index(index: usize, board: &Board) -> Option<Zone>{
        if board[index] != MinesweeperCell::Mine { return None};

        let next_to = board.get_next_to(index, NextToPolicy::EightAround);
        let (empty, bomb) = board.empty_and_mine_count(&next_to);
        if bomb > 0 {return None};

        Some(Zone {minecount:(1..=empty.len()).collect_vec(), cells:empty})
    }
    apply_to_all_squares(board, check_index)
}

// </ZoneRules>

// <ZoneCombineRule>

fn strict_subset(zone1: &Zone, zone2: &Zone) -> Option<Zone>{
    let (only_first, _, only_second) = zone1.sections(zone2);

    if only_first.is_empty(){

        let mut counts = Vec::new();

        if zone2.max() < zone1.min() {panic!("subset has only larger values")}

        for a in &zone1.minecount{
            for b in &zone2.minecount{
                if a <= b {
                    counts.push(b-a);
                }
            }
        }

        return Some(Zone { cells: only_second, minecount: counts })
    }

    if only_second.is_empty(){
        let mut counts = Vec::new();

        if zone1.max() < zone2.min() {panic!("subset has only larger values")}

        for a in &zone1.minecount{
            for b in &zone2.minecount{
                if b <= a {
                    counts.push(a-b);
                }
            }
        }

        return Some(Zone { cells: only_first, minecount: counts })
    }

    None
}


fn add_min_bounds(zone1: &Zone, zone2: &Zone) -> Option<Zone>{
    let (x_spaces, _, z_spaces) = zone1.sections(zone2);

    if x_spaces.is_empty() || z_spaces.is_empty(){
        return None;
    }

    // / a  \
    // [x,[y],z]
    //    \  b /

    let amin = zone1.min();
    let xmax = x_spaces.len();

    if amin > xmax{

        let ymin = amin - xmax;
        let bmax = zone2.max();
        let zmax = bmax - ymin;       

        return Some(Zone{cells:z_spaces, minecount:(0..=zmax).collect()})
    }

    
    let bmin = zone2.min();
    let zmax = z_spaces.len();

    if bmin > zmax{
        let ymin = bmin - zmax;
        let amax = zone1.max();
        let xmax = amax - ymin;

        return Some(Zone{cells:x_spaces, minecount:(0..=xmax).collect()})
    }
    
    None
    
}


fn handle_small_overlaps(zone1: &Zone, zone2: &Zone) -> Option<Zone>{
    let (x_spaces, _, z_spaces) = zone1.sections(zone2);

    if x_spaces.is_empty() || z_spaces.is_empty(){
        return None;
    }

    // / a  \
    // [x,[y],z]
    //    \  b /

    let amin = zone1.min();
    let amax = zone1.max();
    let bmin = zone2.min();
    let bmax = zone2.max();

    if amax < bmin{     
        return Some(Zone{cells:z_spaces, minecount:(bmin-amax..=bmax).collect()})
    }

    
    if bmax < amin{     
        return Some(Zone{cells:x_spaces, minecount:(amin-bmax..=amax).collect()})
    }
    
    None
    
}


// <ZoneCombineRule/>

// <SolveRules>

fn full_zone(zone: &Zone) -> Option<KnownSquares>{
    if zone.count() == zone.min() {
        return Some(KnownSquares {mines:HashSet::from_iter(zone.cells.clone()), safe:HashSet::new()})
    };
    return None
}

fn empty_zone(zone: &Zone) -> Option<KnownSquares>{
    if zone.max() == 0{
        return Some(KnownSquares {mines:HashSet::new(), safe:HashSet::from_iter(zone.cells.clone())})
    }
    return None
}

// </SolveRules>

fn max_min_bound_overlap(zone1: &Zone, zone2: &Zone) -> Option<KnownSquares>{
    let (only_first, _, only_second) = zone1.sections(zone2);

    let z1min = i8::try_from(zone1.min()).unwrap();
    let z2min = i8::try_from(zone2.min()).unwrap();
    let z1max = i8::try_from(zone1.max()).unwrap();
    let z2max = i8::try_from(zone2.max()).unwrap();
    let z1ex = i8::try_from(only_first.len()).unwrap();
    let z2ex = i8::try_from(only_second.len()).unwrap();

    if z2min - z1max == z2ex {return Some(KnownSquares { mines:HashSet::from_iter(only_second), safe:HashSet::from_iter(only_first)})};
    if z1min - z2max == z1ex {return Some(KnownSquares { mines:HashSet::from_iter(only_first), safe:HashSet::from_iter(only_second)})};

    None
}

// <ZoneRuleSolvers>

pub fn vanilla_solver() -> ZoneRuleSolver {
    ZoneRuleSolver {
        zone_rules:vec![add_zone_for_number_v, total_mines_standard],
        zone_combine_rules:vec![handle_small_overlaps, add_min_bounds, strict_subset],
        solve_rules:vec![full_zone, empty_zone],
        solve_rules_two:vec![max_min_bound_overlap]
    }
}

pub fn quad_solver() -> ZoneRuleSolver {
    ZoneRuleSolver {
        zone_rules:vec![add_zone_for_number_v, total_mines_standard, add_q_spaces],
        zone_combine_rules:vec![handle_small_overlaps, add_min_bounds, strict_subset],
        solve_rules:vec![full_zone, empty_zone],
        solve_rules_two:vec![max_min_bound_overlap]
    }
}

pub fn cross_solver() -> ZoneRuleSolver {
    ZoneRuleSolver {
        zone_rules:vec![add_zone_for_number_x, total_mines_standard],
        zone_combine_rules:vec![handle_small_overlaps, add_min_bounds, strict_subset],
        solve_rules:vec![full_zone, empty_zone],
        solve_rules_two:vec![max_min_bound_overlap]
    }
}

pub fn connected_solver() -> ZoneRuleSolver {
    ZoneRuleSolver {
        zone_rules:vec![add_zone_for_number_v, total_mines_standard, add_connected_mines],
        zone_combine_rules:vec![handle_small_overlaps, add_min_bounds, strict_subset],
        solve_rules:vec![full_zone, empty_zone],
        solve_rules_two:vec![max_min_bound_overlap]
    }
}

// </ZoneRuleSolvers>