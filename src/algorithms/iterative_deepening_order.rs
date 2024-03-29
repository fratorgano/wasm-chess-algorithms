// iterative deepening that uses negamax with alpha-beta pruning and quiescence search

use std::vec;

use shakmaty::*;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use instant::Instant;
use crate::ordering;

use crate::evaluate;

pub fn root(fen_str: &str, seed: u64, max_time: u64) -> String {
  let start = Instant::now();
  let fen: fen::Fen = fen_str.parse().unwrap();
  let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
  let mut legals = pos.legal_moves();
  let mut best_moves:Vec<Move> = vec![];
  let mut new_best_moves:Vec<Move> = vec![];
  let mut alpha;
  let mut beta;
  let mut depth = 0;
  let mut best_prev_moves:Vec<Move> = vec![];

  while start.elapsed().as_millis() < max_time.into() || best_moves.len()==0 {
    alpha = -1_000_000;
    beta = 1_000_000;
    best_moves = new_best_moves.clone();
    new_best_moves = vec![];

    // starting from last best move
    if best_prev_moves.len() > 0 {
      let last_best_move = best_prev_moves.last().unwrap().clone();
      // println!("{:?}", last_best_move);
      legals.sort_by(|a, b| {
        if a==&last_best_move && b!=&last_best_move {
          std::cmp::Ordering::Less
        }else {
          ordering::move_ordering(a,b)
        }
      });
      /* println!("{:?}", legals[0]);
      println!("{:?}", last_best_move);
      assert!(legals[0]==last_best_move); */
    }
    // legals.sort_by(|a, b| a == )

    for legal in &legals {
      let new_pos = pos.clone().play(&legal);
      if new_pos.is_ok() {
        let new_fen = fen::Fen::from_position(new_pos.unwrap(), EnPassantMode::Legal).to_string();
        let score_option = iterative_deepening(new_fen.as_str(), seed, depth, -beta, -alpha, max_time, start);
        if score_option.is_none() {
          break;
        }
        let (mut score, mut prev_moves) = score_option.unwrap();
        score = -score;
        // println!("{:?} -> {:?}", san::San::from_move(&pos,&legal).to_string(), score);
        if score >= beta {
          println!("found another move with same score, updating vector");
          new_best_moves.push(legal.clone());
          best_prev_moves = prev_moves.clone();

          break;
          // return san::San::from_move(&pos,&legal).to_string();
        }
        if score > alpha {
          alpha = score;
          new_best_moves = vec![legal.clone()];
          prev_moves.push(legal.clone());
          best_prev_moves = prev_moves.clone();
        }
      }
    }
    println!("Depth {:?} in {:?}ms", depth, start.elapsed().as_millis());
    /* for move_ in &best_prev_moves {
      print!("{:?} ", move_.to_uci(CastlingMode::Standard).to_string());
    }
    println!(""); */
    depth += 1;
  }
  
  let mut rng = SmallRng::seed_from_u64(seed);
  let move_index = rng.gen_range(0..best_moves.len());
  let san_move = san::San::from_move(&pos, &best_moves[move_index]);
  for move_ in &best_prev_moves {
    println!("{:?}", move_.to_uci(CastlingMode::Standard).to_string());
  }

  return san_move.to_string();

}

fn iterative_deepening(fen_str: &str, seed:u64, depth:u64, mut alpha:i64, beta:i64, max_time:u64, start:Instant) -> Option<(i64,Vec<Move>)> {
  if start.elapsed().as_millis() > max_time.into() {
    // if we've reached the max time, return None
    return None;
  }
  if depth == 0 {
    // instead of just returning the score, we call quiescent search
    // return Some(evaluate::evaluate(fen_str));
    return Some((quiescent_search(fen_str, alpha, beta, max_time*5, start), vec![]));
  }
  let fen: fen::Fen = fen_str.parse().unwrap();
  let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
  let mut legals = pos.legal_moves();
  let mut best_score = None;
  let mut best_previous_moves:Option<Vec<Move>> = None;
  let mut rng = SmallRng::seed_from_u64(seed);

  // starting from last best move
  let best_prev_moves = best_previous_moves.clone().unwrap_or(vec![]);
  if best_prev_moves.len() > 0 {
    let last_best_move = best_prev_moves.last().unwrap();
    // println!("{:?}", last_best_move);
    legals.sort_by(|a, b| {
      if a==last_best_move && b!=last_best_move {
        std::cmp::Ordering::Less 
      }else {
        ordering::move_ordering(a,b)
      }
    });
  }
  
  for legal in legals {
    let new_pos = pos.clone().play(&legal);
    if new_pos.is_ok() {
      let new_fen = fen::Fen::from_position(new_pos.unwrap(), EnPassantMode::Legal).to_string();
      let score_option = iterative_deepening(new_fen.as_str(), rng.gen::<u64>(),depth-1, -beta, -alpha, max_time, start);
      if score_option.is_none() {
        return None;
      }
      let (mut score, mut prev_moves) = score_option.unwrap();
      score = -score;
      // updating new best score
      if best_score == None || score > best_score.unwrap() {
        best_score = Some(score);
        prev_moves.push(legal.clone());
        best_previous_moves = Some(prev_moves);
      }
      // pruning
      if score >= beta {
        return Some((beta, best_previous_moves.unwrap()));
      }
      // updating alpha
      if score > alpha {
        best_score = Some(score);
        alpha = score;
      }
    }
  }
  // or allows us to return the evaluation of the position if no legal moves are available, adjusted with the depth so we aim for faster checkmate
  if best_score.is_none() {
    return Some((evaluate::evaluate(fen_str) - depth as i64, vec![]));
  }
  Some((best_score.unwrap(), best_previous_moves.unwrap()))
  // best_score.unwrap_or(evaluate::evaluate(fen_str))
}

pub fn quiescent_search(fen_str: &str, mut alpha:i64, beta:i64, max_time:u64, start:Instant) -> i64 {
  // https://www.chessprogramming.org/Quiescence_Search
  let stand_pat = evaluate::evaluate(fen_str);
  if start.elapsed().as_millis() > max_time.into() {
    // if we've reached the max time, return beta
    return beta;
  }
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
      let score = -quiescent_search(new_fen.as_str(), -beta, -alpha, max_time, start);
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
  const NAME: &str = "iterative deepening";

  // cargo test --release -- --nocapture checkmate_black_wins_in_1 
  #[test]
  #[ignore]
  fn test_exec_time() {
    use std::time::Instant;
    let now = Instant::now();
    println!("[{:?}] {:?}",NAME,root("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 0, 1000));
    let elapsed = now.elapsed();
    println!("[{:?}] Elapsed: {:.2?}",NAME, elapsed);
  }

  #[test]
  fn checkmate_black_wins_in_1() {
    assert_eq!(root("4k3/8/8/8/8/2r5/1q6/5K2 b - - 3 2", 1, 1000), "Rc1");
  }
  #[test]
  fn checkmate_black_wins_in_2() {
    let mov = root("4k3/8/8/8/1qr5/8/8/6K1 b - - 3 2", 1, 2000);
    if !(mov == "Qb2" || mov == "Qd2" || mov == "Rc2") {
      panic!("{}", mov);
    }
  }

  #[test]
  fn checkmate_white_wins_in_1() {
    assert_eq!(root("5k2/2R5/1Q6/8/8/8/8/4K3 w - - 3 3", 1, 1000), "Qb8");
  }

  #[test]
  fn checkmate_white_wins_in_2() {
    let mov = root("5k2/8/1Q6/2R5/8/8/8/4K3 w - - 3 3", 1, 1000);
    if !(mov == "Qb7" || mov == "Qa7" || mov == "Rc7") {
      panic!("{}", mov);
    }
  }
}
