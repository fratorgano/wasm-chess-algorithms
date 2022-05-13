use std::cmp::Ordering;

use shakmaty::*;

pub fn move_ordering(m1:&Move,m2:&Move) -> Ordering {
  if m1.is_capture() && !m2.is_capture() {
    // a capture is always better than a non-capture
    return Ordering::Less;
  } else if !m1.is_capture() && m2.is_capture() {
    // a non-capture is always worst than a capture
    return Ordering::Greater;
  } else if m1.is_capture() && m2.is_capture() {
    // if both are capture, we compare the capture value and capturer value
    return capture_ordering(m1,m2);
  } else {
    // if neither is capture, we prefer promotions 
    if m1.is_promotion() && !m2.is_promotion() {
      return Ordering::Less;
    } else if !m1.is_promotion() && m2.is_promotion() {
      return Ordering::Greater;
    } else {
      // if neither is promotion, we return equal
      return Ordering::Equal;
    }
  }
}

pub fn capture_ordering(m1:&Move,m2:&Move) -> Ordering {
  // both are capture so we sort based on capture value and piece that captured (MVV-LVA)
  let c1 = m1.capture().unwrap();
  let c2 = m2.capture().unwrap();
  let p1 = m1.role();
  let p2 = m2.role();
  if c1.cmp(&c2) == Ordering::Equal {
    return p1.cmp(&p2);
  } else {
    return c2.cmp(&c1);
  }
}
#[cfg(test)]
mod test {

use super::*;
  #[test]
  fn test_move_ordering_mvvlva() {
    let fen = "7k/4P3/8/2r5/1Q1P2p1/8/8/2R4K w - - 0 1";
    let fen: fen::Fen = fen.parse().unwrap();
    let pos: Chess = fen.into_position(CastlingMode::Standard).unwrap();
    let mut legals = pos.legal_moves();
    /* for legal in &legals {
      println!("{:?}", legal);
    } */
    legals.sort_by(move_ordering);
    /* println!("sorted:"); */
    let first = legals.first().unwrap();
    assert_eq!(first.to_string(), "d4xc5");
    /* for legal in &legals {
      println!("{:?}", legal);
    } */
    
  }
}