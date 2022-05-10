/* pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
} */

use serde::{Serialize, Deserialize};
use shakmaty::{Chess, zobrist::ZobristHash, fen::Fen, CastlingMode};

use crate::HashTable;

// zobrish hasher for fen string
pub fn zobrish(fen_str: &str) -> u64 {
    let fen: Fen = fen_str.parse().unwrap();
    let chess: Chess = fen.into_position(CastlingMode::Standard).unwrap();
    chess.zobrist_hash::<u64>()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveAndTable {
  pub mov: String,
  pub table: HashTable,
}
impl MoveAndTable {
  pub fn new(mov: String, table: HashTable) -> MoveAndTable {
    MoveAndTable { mov, table }
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_zobrish_hash() {
    // let mut hasher = ChessHash::new_hasher();
    // chess_hash.hash(&mut hasher);
    // let hash = hasher.finish();
    let hash = zobrish("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(hash, 0x463b96181691fc9c);
  }
}