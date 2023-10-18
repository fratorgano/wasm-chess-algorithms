use std::vec;

use shakmaty::*;//{fen::Fen, Board,Piece,Square, CastlingMode, Chess, Position, san::San};

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use crate::evaluate;

pub fn root(fen_str: &str, seed: u64, depth: u64) -> String {
  let fen: fen::Fen = fen_str.parse().unwrap();
  let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
  let legals = pos.legal_moves();
  let mut best_moves:Vec<Move> = vec![];
  let mut alpha = -1_000_000;
  let beta = 1_000_000;
  for legal in legals {
    let new_pos = pos.clone().play(&legal);
    if new_pos.is_ok() {
      let new_fen = fen::Fen::from_position(new_pos.unwrap(), EnPassantMode::Legal).to_string();
      let score = -negamax_a_b(new_fen.as_str(), seed, depth - 1, -beta, -alpha);
      // println!("{:?} -> {:?}", san::San::from_move(&pos,&legal).to_string(), score);
      if score > beta {
        return san::San::from_move(&pos,&legal).to_string();
      } else if score == beta {
        best_moves.push(legal);
      } else if score > alpha {
        alpha = score;
        best_moves = vec![legal];
      }
    }
  }
  let mut rng = SmallRng::seed_from_u64(seed);
  let move_index = rng.gen_range(0..best_moves.len());
  let san_move = san::San::from_move(&pos, &best_moves[move_index]);
  return san_move.to_string();

}

fn negamax_a_b(fen_str: &str, seed:u64, depth: u64, mut alpha:i64, beta:i64) -> i64 {
  if depth == 0 {
    // instead of just returning the score, we call quiescent search
    // return evaluate::evaluate(fen_str);
    return quiescent_search(fen_str, alpha, beta);
  }
  let fen: fen::Fen = fen_str.parse().unwrap();
  let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
  let legals = pos.legal_moves();
  let mut best_score = None;
  let mut rng = SmallRng::seed_from_u64(seed);
  for legal in legals {
    let new_pos = pos.clone().play(&legal);
    if new_pos.is_ok() {
      let new_fen = fen::Fen::from_position(new_pos.unwrap(), EnPassantMode::Legal).to_string();
      let score = -negamax_a_b(new_fen.as_str(), rng.gen::<u64>(), depth - 1, -beta, -alpha);
      if best_score == None || score > best_score.unwrap() {
        best_score = Some(score);
      }
      if score >= beta {
        return beta;
      }
      if score > alpha {
        best_score = Some(score);
        alpha = score;
      }
    }
  }
  // or allows us to return the evaluation of the position if no legal moves are available, adjusted with the depth so we aim for faster checkmate
  best_score.unwrap_or(evaluate::evaluate(fen_str)-depth as i64)
  // best_score.unwrap_or(evaluate::evaluate(fen_str))
}

pub fn quiescent_search(fen_str: &str, mut alpha:i64, beta:i64) -> i64 {
  // https://www.chessprogramming.org/Quiescence_Search
  let stand_pat = evaluate::evaluate(fen_str);
  if stand_pat >= beta {
    return beta;
  }
  if alpha < stand_pat {
    alpha = stand_pat;
  }

  let fen: fen::Fen = fen_str.parse().unwrap();
  let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
  let capture_moves = pos.capture_moves();
  
  for capture in capture_moves {
    let new_pos = pos.clone().play(&capture);
    if new_pos.is_ok() {
      let new_fen = fen::Fen::from_position(new_pos.unwrap(), EnPassantMode::Legal).to_string();
      let score = -quiescent_search(new_fen.as_str(), -beta, -alpha);
      if score >= beta {
        return beta;
      }
      if score > alpha {
        alpha = score;
      }
    }
  }
  alpha
}


#[cfg(test)]
mod test {
  use super::*;
  const NAME: &str = "negamax_a_b_quiescent";

  // cargo test --release -- --nocapture checkmate_black_wins_in_1 
  #[test]
  #[ignore]
  fn test_exec_time() {
    use std::time::Instant;
    let now = Instant::now();
    println!("[{:?}] {:?}",NAME,root("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 0, 5));
    let elapsed = now.elapsed();
    println!("[{:?}] Elapsed: {:.2?}",NAME, elapsed);
  }

  #[test]
  fn checkmate_black_wins_in_1() {
    assert_eq!(root("4k3/8/8/8/8/2r5/1q6/5K2 b - - 3 2", 1, 3), "Rc1");
  }
  #[test]
  fn checkmate_black_wins_in_2() {
    let mov = root("4k3/8/8/8/1qr5/8/8/6K1 b - - 3 2", 1, 4);
    if !(mov == "Qb2" || mov == "Qd2" || mov == "Rc2") {
      panic!("{}", mov);
    }
  }

  #[test]
  fn checkmate_white_wins_in_1() {
    assert_eq!(root("5k2/2R5/1Q6/8/8/8/8/4K3 w - - 3 3", 1, 4), "Qb8");
  }

  #[test]
  fn checkmate_white_wins_in_2() {
    let mov = root("5k2/8/1Q6/2R5/8/8/8/4K3 w - - 3 3", 1, 4);
    if !(mov == "Qb7" || mov == "Qa7" || mov == "Rc7") {
      panic!("{}", mov);
    }
  }
}