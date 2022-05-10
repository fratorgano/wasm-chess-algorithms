use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::zobrish;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionInfo {
  pub depth: u64,
  pub score: i64,
  pub mov: String,
}
impl PositionInfo {
  pub fn new(depth: u64, score: i64, mov: String) -> PositionInfo {
    PositionInfo { depth, score, mov }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HashTable {
  pub table: HashMap<String, PositionInfo>,
}
impl HashTable {
  pub fn new() -> HashTable {
    HashTable {
      table: HashMap::new(),
    }
  }
  pub fn insert(&mut self, fen_str: &str, pos_info: PositionInfo) -> Option<PositionInfo> {
    self.table.insert(zobrish(fen_str).to_string(), pos_info)
  }
  pub fn find(&mut self, fen_str: &str, depth: u64) -> Option<PositionInfo> {
    let pos_info = self.table.get(&zobrish(fen_str).to_string());
    if pos_info.is_some() {
      let pos_info = pos_info.unwrap();
      if pos_info.depth >= depth {
        return Some(pos_info.clone());
      }
    }
    return None;
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::{zobrish};

  // cargo test --release -- --nocapture checkmate_black_wins_in_1 
  #[test]
  fn test_creation_input() {
    let mut hashtable = HashTable::new();
    let position_info = PositionInfo::new(0, 0, "".to_string());
    let zobrish_hash = zobrish("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    hashtable.table.insert(zobrish_hash.to_string(), position_info);
    println!("{:?}", hashtable);
  }
}