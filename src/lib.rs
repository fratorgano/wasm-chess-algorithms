mod utils;
mod algorithms;
mod evaluate;
mod ordering;

use wasm_bindgen::prelude::*;
use crate::algorithms::hashtable::{HashTable};
use crate::utils::{zobrish, MoveAndTable};

// wasm-pack build --target nodejs --out-dir /home/fra/SocketChess/node_modules/wasm-chess-algorithms

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/* #[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let greeting = format!("Hello {} to wasm-game-of-life!", name);
    alert(&greeting);
} */

#[wasm_bindgen]
pub fn random_move(fen_str: &str, seed: u64) -> String {
    return algorithms::random::random_move(fen_str, seed);
}

#[wasm_bindgen]
pub fn negamax_move(fen_str: &str, seed: u64, depth: u64) -> String {
    return algorithms::negamax::negamax_root(fen_str, seed, depth)
}

#[wasm_bindgen]
pub fn negamax_a_b_move(fen_str: &str, seed: u64, depth: u64) -> String {
    return algorithms::negamax_a_b::negamax_a_b_root(fen_str, seed, depth)
}

#[wasm_bindgen]
pub fn negamax_a_b_table_move(fen_str: &str, seed: u64, depth: u64, lastres: JsValue) -> JsValue {
    let move_and_table = lastres.into_serde::<MoveAndTable>();
    // table = if move_and_table is ok, unwrap it otherwise use none
    let table = match move_and_table {
        Ok(mt) => Some(mt.table),
        Err(_) => None,
    };
    /* let string = format!("[Rust-lib] {}", if table.is_none() { "None" } else { "Some" });
    web_sys::console::log_1(&string.into()); */
    let fun_res = algorithms::negamax_a_b_table::root(fen_str, seed, depth, table);
    let data = JsValue::from_serde(&fun_res);
    if data.is_err() {
        panic!("Error during negamax_a_b_table_move {:?}", data.err().unwrap());
    }
    return data.unwrap();
}

#[wasm_bindgen]
pub fn negamax_a_b_quiescent(fen_str: &str, seed: u64, depth: u64) -> String {
    return algorithms::negamax_a_b_quiescent::root(fen_str, seed, depth)
}

#[wasm_bindgen]
pub fn iterative_deepening(fen_str: &str, seed: u64, max_time:u64) -> String {
    return algorithms::iterative_deepening::root(fen_str, seed, max_time)
}

#[wasm_bindgen]
pub fn iterative_deepening_order(fen_str: &str, seed: u64, max_time:u64) -> String {
    return algorithms::iterative_deepening_order::root(fen_str, seed, max_time)
}

/* #[wasm_bindgen]
pub fn give() -> JsValue {
    let mut hashtable = HashTable::new();
    let position_info = PositionInfo::new(0, 0, "".to_string());
    let fen_hash = zobrish("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    hashtable.table.insert(fen_hash.to_string(), position_info);
    let data = JsValue::from_serde(&hashtable);
    if data.is_err() {
        panic!("{:?}", data.err().unwrap());
    }
    return data.unwrap();
}

#[wasm_bindgen]
pub fn give_back(data: &JsValue) {
    let hashtable = data.into_serde::<HashTable>().unwrap();
    println!("{:?}", hashtable);
} */
