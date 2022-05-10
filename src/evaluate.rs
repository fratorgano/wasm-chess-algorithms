use shakmaty::*;
use shakmaty::fen::Fen;

const KING_VALUE: i64 = 10_000;
const QUEEN_VALUE: i64 = 1_000;
const ROOK_VALUE: i64 = 500;
const BISHOP_VALUE: i64 = 350;
const KNIGHT_VALUE: i64 = 300;
const PAWN_VALUE: i64 = 100;

pub fn evaluate(fen_str: &str) -> i64 {
  let fen: Fen = fen_str.parse().unwrap();
  let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
  let board_iter: board::IntoIter = pos.board().to_owned().into_iter();
  let mut score: i64 = 0;
  let who_moves = pos.turn();

  // if game is stalemate, return 0

  // if game is done
  if pos.is_game_over()  {
    let outcome = pos.outcome().unwrap();
    // if outcome is checkmate and white won, set score = KING_VALUE
    // if outcome is checkmate and black won, set score = -KING_VALUE
    // if outcome is draw, set score = 0
    match outcome {
      Outcome::Decisive{winner: Color::White} => {
        // println!("Black lost");
        score = KING_VALUE
      },
      Outcome::Decisive{winner: Color::Black} => {
        // println!("White lost");
        score = -KING_VALUE
      },
      Outcome::Draw => score= 0,
    }
    // -KING_VALUE because the who_moves side lost
    
  } else {
    // if game is not over, evaluate board
    for (_square, piece) in board_iter {
      score += piece_value(piece);
    }
  }


  if who_moves == Color::White {
    return score;
  } else {
    return -score;
  }
}


pub fn piece_value(piece: Piece) -> i64 {
  let value = match piece.role {
    Role::King => KING_VALUE,
    Role::Queen => QUEEN_VALUE,
    Role::Rook => ROOK_VALUE,
    Role::Bishop => BISHOP_VALUE,
    Role::Knight => KNIGHT_VALUE,
    Role::Pawn => PAWN_VALUE
  };
  if piece.color.is_white() {
    value
  } else {
    -value
  }
}
// evaluate("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn starting_pos_eval() {
    assert_eq!(evaluate("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), 0);
  }

  #[test]
  fn pos_eval_white_turn_white_advantage() {
    // white turn
    // white has one pawn more
    assert_eq!(evaluate("rnbqkbnr/pppp1ppp/8/4P3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1"), 100);
  }

  #[test]
  fn pos_eval_white_turn_black_advantage() {
    // white turn
    // white has one pawn less
    assert_eq!(evaluate("rnbqkbnr/pppp1ppp/8/8/3p4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1"), -100);
  }

  #[test]
  fn pos_eval_black_turn_white_advantage() {
    // white turn
    // white has one pawn more
    assert_eq!(evaluate("rnbqkbnr/pppp1ppp/8/4P3/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 2"), -100);
  }

  #[test]
  fn pos_eval_black_turn_black_advantage() {
    // black turn
    // white has one pawn less
    assert_eq!(evaluate("rnbqkbnr/pppp1ppp/8/8/3pP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 3"), 100);
  }

  #[test]
  fn pos_eval_checkmate_white_wins() {
    // black turn
    // white wins
    assert_eq!(evaluate("1Q2k3/2R5/8/8/8/8/8/4K3 b - - 1 1"), -10_000);
  }

  #[test]
  fn pos_eval_checkmate_black_wins() {
    // white turn
    // black wins
    assert_eq!(evaluate("4k3/8/8/8/8/8/1q6/2r1K3 w - - 2 2"), -10_000);
  }
}