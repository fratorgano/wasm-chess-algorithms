// iterative deepening that uses negamax with alpha-beta pruning and quiescence search
use std::vec;

use shakmaty::*;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use instant::Instant;

use crate::algorithms::hashtable::PositionInfo;
use crate::evaluate;
use crate::HashTable;
use crate::utils::MoveAndTable;

pub fn root(fen_str: &str, seed: u64, max_time: u64, table: Option<HashTable>) -> MoveAndTable {
  let start = Instant::now();
  let fen: fen::Fen = fen_str.parse().unwrap();
  let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
  let legals = pos.legal_moves();
  let mut best_moves:Vec<String> = vec![];
  let mut new_best_moves:Vec<String> = vec![];
  let mut alpha;
  let mut beta;
  let mut depth = 0;
  let mut table = table.unwrap_or_else(|| HashTable::new());

  while start.elapsed().as_millis() < max_time.into() || best_moves.len()==0 {
    alpha = -1_000_000;
    beta = 1_000_000;
    best_moves = new_best_moves.clone();
    new_best_moves = vec![];

    for legal in &legals {
      let new_pos = pos.clone().play(&legal);
      if new_pos.is_ok() {
        let new_fen = fen::Fen::from_position(new_pos.unwrap(), EnPassantMode::Legal).to_string();
        let score_option = iterative_deepening(new_fen.as_str(), seed, depth, -beta, -alpha, max_time, start, &mut table);
        if score_option.is_none() {
          break;
        }
        let score = -score_option.unwrap();
        // println!("{:?} -> {:?}", san::San::from_move(&pos,&legal).to_string(), score);
        // store score in table 
        table.insert(&new_fen, PositionInfo {
          mov: san::San::from_move(&pos,&legal).to_string(),
          score,
          depth,
        });
        if score >= beta {
          let san_move = san::San::from_move(&pos,&legal).to_string();
          new_best_moves.push(san_move);
          break;
          // return san::San::from_move(&pos,&legal).to_string();
        }
        if score > alpha {
          alpha = score;
          let san_move = san::San::from_move(&pos,&legal).to_string();
          new_best_moves = vec![san_move];
        }
      }
    }
    println!("Depth {:?} in {:?}ms", depth, start.elapsed().as_millis());
    depth += 1;
  }
  
  let mut rng = SmallRng::seed_from_u64(seed);
  let move_index = rng.gen_range(0..best_moves.len());
  let san_move = &best_moves[move_index];
  // println!("{:?}", &table);
  MoveAndTable::new(san_move.to_string(), table)
}

fn iterative_deepening(fen_str: &str, seed:u64, depth:u64, mut alpha:i64, beta:i64, max_time:u64, start:Instant, table: &mut HashTable) -> Option<i64> {
  if start.elapsed().as_millis() > max_time.into() {
    // if we've reached the max time, return None
    return None;
  }
  // if there's an entry in the table with a depth greater than the current depth, return it
  let table_entry = table.find(&fen_str, depth);
  if let Some(entry) = table_entry {
    return Some(entry.score /* - (entry.depth-depth) as i64 */);
  }
  
  if depth == 0 {
    // instead of just returning the score, we call quiescent search
    // return Some(evaluate::evaluate(fen_str));
    return Some(quiescent_search(fen_str, alpha, beta));
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
      let score_option = iterative_deepening(new_fen.as_str(), rng.gen::<u64>(),depth-1, -beta, -alpha, max_time, start, table);
      if score_option.is_none() {
        return None;
      }
      let score = -score_option.unwrap();
      if best_score == None || score > best_score.unwrap() {
        best_score = Some(score);
      }
      if score >= beta {
        return Some(beta);
      }
      if score > alpha {
        best_score = Some(score);
        alpha = score;
      }
    }
  }
  // or allows us to return the evaluation of the position if no legal moves are available, adjusted with the depth so we aim for faster checkmate
  Some(best_score.unwrap_or(evaluate::evaluate(fen_str)-depth as i64))
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
  const NAME: &str = "iterative deepening table2";

  // cargo test --release -- --nocapture checkmate_black_wins_in_1 
  #[test]
  #[ignore]
  fn test_exec_time() {
    use std::time::Instant;
    let now = Instant::now();
    let res = root("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 0, 1000,None);
    println!("[{:?}] {:?}",NAME,res.mov);
    let elapsed = now.elapsed();
    println!("[{:?}] Elapsed: {:.2?}",NAME, elapsed);

    let now = Instant::now();
    let res = root("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 0, 1000,Some(res.table));
    println!("[{:?}] {:?}",NAME,res.mov);
    let elapsed = now.elapsed();
    println!("[{:?}] Elapsed: {:.2?}",NAME, elapsed);

    let now = Instant::now();
    let res = root("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 0, 1000,Some(res.table));
    println!("[{:?}] {:?}",NAME,res.mov);
    let elapsed = now.elapsed();
    println!("[{:?}] Elapsed: {:.2?}",NAME, elapsed);
  }

  #[test]
  fn checkmate_black_wins_in_1() {
    assert_eq!(root("4k3/8/8/8/8/2r5/1q6/5K2 b - - 3 2", 1, 1000, None).mov, "Rc1");
  }
  #[test]
  fn checkmate_black_wins_in_2() {
    let mov = root("4k3/8/8/8/1qr5/8/8/6K1 b - - 3 2", 1, 1000, None).mov;
    if !(mov == "Qb2" || mov == "Qd2" || mov == "Rc2") {
      panic!("{}", mov);
    }
  }

  #[test]
  fn checkmate_white_wins_in_1() {
    assert_eq!(root("5k2/2R5/1Q6/8/8/8/8/4K3 w - - 3 3", 1, 1000, None).mov, "Qb8");
  }

  #[test]
  fn checkmate_white_wins_in_2() {
    let mov = root("5k2/8/1Q6/2R5/8/8/8/4K3 w - - 3 3", 1, 1000, None).mov;
    if !(mov == "Qb7" || mov == "Qa7" || mov == "Rc7") {
      panic!("{}", mov);
    }
  }
}
