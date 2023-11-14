use crate::linear_programming_solvers::{*};
use crate::board::Board;

//qmlbnxt

pub fn create_constraint_set_qm(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_m(board), create_q_added_constraint_set(board))
}

pub fn create_constraint_set_ql(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_l(board), create_q_added_constraint_set(board))
}

// QW

pub fn create_constraint_set_qn(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_n(board), create_q_added_constraint_set(board))
}

pub fn create_constraint_set_qx(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_x(board), create_q_added_constraint_set(board))
}

// QP

// QE

// 
// All of C
// 

pub fn create_constraint_set_tm(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_m(board), create_t_added_constraint_set(board))
}

pub fn create_constraint_set_tl(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_l(board), create_t_added_constraint_set(board))
}

// TW

pub fn create_constraint_set_tn(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_n(board), create_t_added_constraint_set(board))
}

pub fn create_constraint_set_tx(board:&Board) -> ConstraintSet {
    combine_constraint_sets(create_constraint_set_x(board), create_t_added_constraint_set(board))
}

// TP

// TE

// 
// All of O
// 

// 
// All of D
// 

// 
// All of S
// 

pub fn create_constraint_set_bm(board:&Board) -> ConstraintSet {
    combine_constraint_sets(combine_constraint_sets(create_constraint_set_multiple_mines(board), create_constraint_set_minecount_b(board)), create_b_added_constraint_set(board))
}

pub fn create_constraint_set_bl(board:&Board) -> ConstraintSet {
    combine_constraint_sets(combine_constraint_sets(create_constraint_set_liar_mines(board), create_constraint_set_minecount_b(board)), create_b_added_constraint_set(board))
}

// TW

pub fn create_constraint_set_bn(board:&Board) -> ConstraintSet {
    combine_constraint_sets(combine_constraint_sets(create_constraint_set_negation_mines(board), create_constraint_set_minecount_b(board)), create_b_added_constraint_set(board))
}

pub fn create_constraint_set_bx(board:&Board) -> ConstraintSet {
    combine_constraint_sets(combine_constraint_sets(create_constraint_set_cross_mines(board), create_constraint_set_minecount_b(board)), create_b_added_constraint_set(board))
}

// TP

// TE