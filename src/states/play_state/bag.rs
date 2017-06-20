extern crate rand;
use self::rand::Rng;

use super::tetromino::{PieceType, Piece, u8_to_piece_type};

/// A bag of `Pieces`. Takes care of dispensing, refilling and giving a peek at
/// the next piece.
pub struct PieceBag {
    queue: Vec<PieceType>,
}

impl PieceBag {
    pub fn new() -> Self {
        PieceBag {
            queue: PieceBag::generate_full_bag()
        }
    }

    /// Takes the next piece from the bag. If the bag is empty after removing a
    /// piece, refill it.
    pub fn take_piece(&mut self) -> Piece {
        let next_piece_shape = self.queue.remove(0);

        // if bag is now empty, generate new bag
        if self.queue.is_empty() {
            self.queue = PieceBag::generate_full_bag();
        }

        Piece::new(next_piece_shape)
    }

    /// Returns the next piece in the bag without actually removing it from the
    /// bag.
    pub fn peek_at_next_piece(&self) -> Piece {
        let next_piece_shape = self.queue.first().expect("Could not peek into PieceBag");

        Piece::new_from_ref(next_piece_shape)
    }

    /// Generates a a full bag of 7 pieces. This is a static function rather
    /// than a method so we can fill the bag in the `new()` function.
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
