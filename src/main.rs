#![allow(dead_code, unused_imports)] // Suppress warning dead_code and unused_imports for debugging purposes.
mod bdd;
mod input;

use crate::bdd::bdd_graph::*;
use crate::bdd::bdd_manager::*;
use crate::input::boolean_function::*;
use crate::input::parser::*;

fn main() {
    let input = parse_file("debug_input/easy1.dimacs").unwrap();
    let input_symbols = BooleanFunction::new_cnf_formula(input);
    let mgr = BDDManager::new_from_cnf(input_symbols);
    println!("{:?}", mgr.bdd)
}
