use crate::piece::Piece;
use permutohedron::Heap;

pub fn generate_perm_iter<'a>(pieces: &'a mut Vec<Piece>) 
    -> Heap<Vec<Piece>, Piece>
{
    Heap::new(pieces)
}