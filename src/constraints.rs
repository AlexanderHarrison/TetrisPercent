use crate::fieldmatrix::FieldMatrix;
use crate::field::{PercentageOptions};
use crate::piece::Piece;

#[derive(Clone, Debug)]
enum Constraint {
    Just(Piece),
    Before(Box<Constraint>, Piece),
    Or(Box<Constraint>, Box<Constraint>),
    And(Box<Constraint>, Box<Constraint>)
}

pub fn generate_constraints(
    pieces: &Vec<Piece>,
    field: &FieldMatrix,
    options: PercentageOptions,
) -> Constraint {
    use Constraint::*;

    let test_field = field.clone();
    
}