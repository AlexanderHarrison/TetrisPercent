use crate::fieldmatrix::FieldMatrix;
use crate::perm_gen::generate_perm_iter;
use crate::piece::{piece_can_be_placed, place_piece_on_field, Piece};

pub const BOTTOM_ROW_DISCARD_COUNT: usize = 1;

#[derive(Copy, Clone)]
pub struct PercentageOptions {
    pub hold: bool,
    pub spin: bool,
    pub soft_drop: bool,
}

impl PercentageOptions {
    pub fn new(hold: bool, soft_drop: bool, spin: bool) -> Self {
        PercentageOptions {
            hold,
            spin,
            soft_drop,
        }
    }
}

pub fn find_percentage(
    base_field: FieldMatrix,
    pieces: Vec<Piece>,
    options: PercentageOptions,
) -> f64 {
    let mut perm_pieces = pieces.clone();
    let permutations = generate_perm_iter(&mut perm_pieces);
    let mut permutation_count: u64 = 0;

    let works_perms_no_hold = permutations
        .filter(|perm| {
            permutation_count += 1;
            permutation_works(&base_field, &perm, options)
        })
        .collect::<Vec<Vec<Piece>>>();

    let works_count = if options.hold {
        crate::hold_comp::compute_with_hold(pieces, works_perms_no_hold)
    } else {
        works_perms_no_hold.len() as u64
    };

    works_count as f64 / permutation_count as f64 * 100.0
}

fn permutation_works(
    base_field: &FieldMatrix,
    piece_perm: &Vec<Piece>,
    options: PercentageOptions,
) -> bool {
    let mut field = base_field.clone();
    for piece in piece_perm.iter() {
        if piece_can_be_placed(*piece, &field, options) {
            place_piece_on_field(*piece, &mut field);
        } else {
            return false;
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

pub fn discard_bottom(mut field: FieldMatrix) -> (FieldMatrix, Vec<[u8; 10]>) {
    let mut old_rows = Vec::new();

    for i in 0..BOTTOM_ROW_DISCARD_COUNT {
        let u_index = (23 - i) as usize;

        old_rows.push(field[u_index].clone());

        for block in field[u_index].iter_mut() {
            *block = 0;
        }
    }

    (field, old_rows)
}
