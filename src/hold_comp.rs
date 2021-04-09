use crate::piece::Piece;
use permutohedron::Heap;

struct SmallPerm {
    pieces: Vec<u8>
}

impl SmallPerm {
    pub fn origin(size: usize) -> Self {
        Self {
            pieces: vec![0; size]
        }
    }
}

// works_without_hold contains the indexes of the permutations that work
pub fn compute_with_hold(pieces: Vec<Piece>, works_without_hold: Vec<usize>) -> u64 {
    let origin = SmallPerm::origin(pieces.len() - 1);
}
