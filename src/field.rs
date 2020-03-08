use crate::fieldmatrix::FieldMatrix;
use crate::piece::{Piece, piece_can_be_placed, place_piece_on_field};
use crate::perm_gen::generate_perm_iter;

#[derive(Copy, Clone)]
pub struct PercentageOptions {
    pub hold: bool,
    pub spin: bool,
    pub soft_drop: bool,
}

impl PercentageOptions {
    pub fn new() -> Self {
        PercentageOptions {
            hold: false,
            spin: false,
            soft_drop: false,
        }
    }

    pub fn hold(mut self, hold: bool) -> Self {
        self.hold = hold;
        self
    }

    pub fn spin(mut self, spin: bool) -> Self {
        self.spin = spin;
        self
    }
}

pub fn find_percentage(
    base_field: FieldMatrix,
    mut pieces: Vec<Piece>,
    options: PercentageOptions,
) -> Result<f64, &'static str> {
    let permutations = generate_perm_iter(&mut pieces);
    
    let mut works_count = 0.0;
    let mut does_not_work_count = 0.0;

    for permutation in permutations.into_iter() {
        
        if permutation_works(&base_field, permutation, options) {
            works_count += 1.0;
        } else {
            does_not_work_count += 1.0;
        }
    }

    //println!("{}, {}", works_count, does_not_work_count);

    Ok(works_count / (works_count + does_not_work_count) * 100.0)
}

fn permutation_works(
    base_field: &FieldMatrix,
    piece_perm: Vec<Piece>,
    options: PercentageOptions
) -> bool {
    /*if !options.hold && !options.spin {
        

        false//
    } else {
        false//
    }*/
    let mut field = base_field.clone();
    for piece in piece_perm.iter() {
        if piece_can_be_placed(
            *piece,
            &field,
            options,
        ) {
            place_piece_on_field(*piece, &mut field);
        } else {
            return false
        }
    }

    true
}

pub fn split_color(matrix: FieldMatrix) -> (FieldMatrix, FieldMatrix) {
    // -> (no_color, color)
    (uncolor(matrix.clone()), get_color(matrix))
}

fn uncolor(mut matrix: FieldMatrix) -> FieldMatrix {
    for n in matrix.iter_mut().flatten() {
        if *n != 8 {
            *n = 0
        }
    }
    matrix
}

fn get_color(mut matrix: FieldMatrix) -> FieldMatrix {
    for n in matrix.iter_mut().flatten() {
        if *n >= 8 {
            *n = 0
        }
    }
    matrix
}