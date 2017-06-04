extern crate rand;
use self::rand::Rng;

use super::tetromino::{PieceType, Piece, u8_to_piece_type};

// piece bag needs to create a full bag (7 pieces in a queue)
// it dispenses pieces when asked for them
// if there are no pieces left after dispensing one, fill itself up
// it can peek ahead to see what the next piece is
pub struct PieceBag {
    queue: Vec<PieceType>,
}

impl PieceBag {
    pub fn new() -> Self {
        PieceBag {
            queue: PieceBag::generate_full_bag()
        }
    }

    pub fn take_piece(&mut self) -> Piece {
        let next_piece_shape = self.queue.remove(0);

        // if bag is now empty, generate new bag
        if self.queue.is_empty() {
            self.queue = PieceBag::generate_full_bag();
        }

        Piece::new(next_piece_shape)
    }

    fn generate_full_bag() -> Vec<PieceType> {
        let mut pieces: Vec<PieceType> = Vec::new();
        for _ in 0 .. 7 {
            let num = rand::thread_rng().gen_range(0, 7);
            let mut shape = u8_to_piece_type(num).unwrap();

            // TODO: better random pieces
            // in the end, we want 1 of each piece
            while pieces.contains(&shape) {
                let num = rand::thread_rng().gen_range(0, 7);
                shape = u8_to_piece_type(num).unwrap();
            }

            // TODO: debug line, remove / put behind 'debug' flag
            //println!("Adding piece: {:?}", shape);

            pieces.push(shape);
        }

        pieces
    }
}
