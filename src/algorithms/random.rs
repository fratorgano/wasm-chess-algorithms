use shakmaty::{fen::Fen, CastlingMode, Chess, Position, san::San};

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub fn random_move(fen_str: &str, seed: u64) -> String {
  let mut rng = SmallRng::seed_from_u64(seed);

  let rand_num: f64 = rng.gen();

  let fen: Fen = fen_str.parse().unwrap();
  let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
  let legals = pos.legal_moves();
  let mov = &legals[(rand_num * legals.len() as f64) as usize];
  let san_move = San::from_move(&pos, &mov);
  // alert(san_move.to_string().as_str());
  return san_move.to_string();
}