use crate::fieldmatrix::FieldMatrix;
use crate::field::PercentageOptions;
use std::fmt::{self, Write, Display, Formatter};

pub mod piece_col;
use piece_col::{PieceCollision, I, O, S, Z, L, J, T};

pub fn piece_type_to_fumen_index(piece: PieceType) -> u8 {
    match piece {
        PieceType::I => 1,
        PieceType::L => 2,
        PieceType::O => 3,
        PieceType::Z => 4,
        PieceType::T => 5,
        PieceType::J => 6,
        PieceType::S => 7,
    }
}

pub fn fumen_index_to_piece_type(i: u8) -> Result<PieceType, &'static str> {
    match i {
        1 => Ok(PieceType::I),
        2 => Ok(PieceType::L),
        3 => Ok(PieceType::O),
        4 => Ok(PieceType::Z),
        5 => Ok(PieceType::T),
        6 => Ok(PieceType::J),
        7 => Ok(PieceType::S),
        _ => Err("incorrect index"),
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    S,
    Z,
    L,
    J,
    T,
    O,
    I,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", 
            match self {
                PieceType::S => "S",
                PieceType::Z => "Z",
                PieceType::O => "O",
                PieceType::I => "I",
                PieceType::T => "T",
                PieceType::L => "L",
                PieceType::J => "J",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rotation {
    Normal,
    Right,
    Left,
    Double,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub rotation: Rotation,
    pub position: (isize, isize),
}

impl Piece {
    pub fn collision(&self) -> PieceCollision {       
        let mut piece = match self.piece_type {
            PieceType::I => I.clone(),
            PieceType::S => S.clone(),
            PieceType::Z => Z.clone(),
            PieceType::O => O.clone(),
            PieceType::L => L.clone(),
            PieceType::J => J.clone(),
            PieceType::T => T.clone(),
        };
        piece.set_rotation(self.rotation);
        piece
    }

    pub fn clone_with_offset(&self, dx: isize, dy: isize) -> Piece {
        let (x, y) = self.position;
        let mut new = self.clone();
        new.position = (x - dx, y - dy);
        new
    }
}

pub fn color_field_to_pieces<'a>(field: FieldMatrix)
    -> Result<Vec<Piece>, String> 
{
    let mut piece_possibilities = Vec::new();
    let rotations = [
        Rotation::Normal,
        Rotation::Right,
        Rotation::Double,
        Rotation::Left
    ];

    for (y, row) in field.iter().enumerate() {
        for (x, fumen_index) in row.iter().enumerate() {
            if *fumen_index == 0 {
                continue
            }

            let piece_type = fumen_index_to_piece_type(*fumen_index)?;
            let (offset_x, offset_y) = piece_check_offset(piece_type);
            
            let mut piece = Piece {
                piece_type,
                rotation: Rotation::Normal,
                position: (x as isize + offset_x, y as isize + offset_y),
            };

            
            // Some pieces have rotational symmetry, so only take the
            // unique iterations
            for rot in 
                rotations.iter().take(
                    get_rotation_times(piece_type)) 
            {
                piece.rotation = *rot;
                if piece_fits_over(piece, *fumen_index, &field)
                    .unwrap_or_else(|| false)
                {
                    piece_possibilities.push(piece.clone());
                }
            }
        }
    }

    // parse piece possibilies for overlapping pieces
    let ambiguous_points = find_ambiguous_points(&piece_possibilities);
    if ambiguous_points.len() > 0 {
        let mut error_string = String::from("Ambiguous points at:\n");
        for point in ambiguous_points.iter() {
            let (x, y) = point;
            writeln!(error_string, "x: {}, y: {}", x, y).unwrap();
        }
        return Err(error_string)
    }

    // points with no piece covering
    let unused_points = find_unused_points(&piece_possibilities, &field); 
    if unused_points.len() > 0 {
        let mut error_string = String::from("Unused points at:\n");
        
        for point in unused_points.iter() {
            let (x, y) = point;
            writeln!(error_string, "x: {}, y: {}", x, y).unwrap();
        }
        return Err(error_string)
    }

    Ok(piece_possibilities)
}

fn find_ambiguous_points(pieces: &Vec<Piece>) -> Vec<(usize, usize)> {   
    let mut temp_field = [[0; 10]; 24];
    for piece in pieces.iter() {
        for (x, y) in piece_block_positions(*piece).unwrap().iter() {
            temp_field[*y][*x] += 1;
        }
    }
    
    temp_field.iter()
        .flatten()
        .enumerate()
        .filter(|(_, n)| **n > 1)
        .map(|(i, _)| {
            (i % 10, i / 10)
        })
        .collect::<Vec<(usize, usize)>>()
}

pub fn find_unused_points(pieces: &Vec<Piece>, field: &FieldMatrix) 
    -> Vec<(usize, usize)> 
{
    let mut test_field = field.clone();

    // get rid of grey blocks
    for n in test_field.iter_mut().flatten().filter(|n| **n == 1) {
        *n = 0;
    }

    //get rid of blocks used by pieces
    for piece in pieces.iter() {
        let block_positions = piece_block_positions(*piece).unwrap_or_default();
        for (x, y) in block_positions.iter() {
            test_field[*y][*x] = 0;
        }
    }

    // any leftover blocks are unused
    test_field.iter()
        .flatten()
        .enumerate()
        .filter(|(_, n)| **n > 0)
        .map(|(i, _)| {
            (i % 10, i / 10)
        })
        .collect::<Vec<(usize, usize)>>()
}

pub fn piece_fits_over(
    piece: Piece,
    cover_type: u8,
    field: &FieldMatrix
) -> Option<bool> {
    //tests if piece is completely covering some value    


    for (dx, dy) in piece_block_positions(piece)?.into_iter() {
        let x = dx as isize;
        let y = dy as isize;
        
        if x < 0 || y < 0 {
            return None
        }
        
        let x = x as usize;
        let y = y as usize;
        
        if !inbounds(x, y) {
            return None
        }
        
        if field[y][x] != cover_type {
            return Some(false)
        }
    }
    
    Some(true)
}

pub fn piece_block_positions(piece: Piece) -> Option<Vec<(usize, usize)>> {
    let col = piece.collision();
    let size = col.size as isize;
    let (x, y) = piece.position;
    let mut positions = Vec::new();
    for (i, _) in col
        .flat_iter()
        .enumerate()
        .filter(|(_, n)| **n == 1)
    {
        let dx = (i as isize % size) + x;
        let dy = i as isize / size + y;
        
        if !signed_inbounds(dx, dy) { return None }

        positions.push((dx as usize, dy as usize));
    }
    Some(positions)
}

fn get_rotation_times(piece: PieceType) -> usize {
    match piece {
        PieceType::O => 1,
        PieceType::S
        | PieceType::Z
        | PieceType::I => 2,
        _ => 4
    }
}

pub fn piece_can_be_placed(
    piece: Piece,
    base_field: &FieldMatrix,
    options: PercentageOptions,
) -> bool {

    if can_harddrop(piece, &base_field) {
        return true
    }
    
    if options.soft_drop {
        softdrop_stem_check(piece, &base_field);
    }

    false
}

fn softdrop_stem_check(base_piece: Piece, base_field: &FieldMatrix) -> bool {
    let mut left_positions: Vec<Piece> = Vec::new();
    let mut right_positions: Vec<Piece> = Vec::new();
    let mut stem_positions: Vec<Piece> = Vec::new();
    stem_positions.push(base_piece);
    //let mut spin_positions: Vec<Piece> = Vec::new();
    
    let mut test_piece_left = |prev_piece: Piece| -> Option<(bool, Piece)> {
        let test_piece = prev_piece.clone_with_offset(-1, 0);

        if piece_fits_over(test_piece, 0, &base_field).unwrap_or_default() {
            if can_harddrop(test_piece, &base_field) {
                return Some((true, test_piece))
            }
            left_positions.push(test_piece);
            return Some((false, test_piece))
        }
        None
    };

    let mut test_piece_right = |prev_piece: Piece| -> Option<(bool, Piece)> {

        let test_piece = prev_piece.clone_with_offset(1, 0);

        if piece_fits_over(test_piece, 0, &base_field).unwrap_or_default() {
            if can_harddrop(test_piece, &base_field) {
                return Some((true, test_piece))
            }
            right_positions.push(test_piece);
            return Some((false, test_piece))
        }
        None
    };

    let test_piece_up =  |prev_piece: Piece| -> (bool, Piece) {
        let test_piece = prev_piece.clone_with_offset(0, 1);
        (
            piece_fits_over(test_piece, 0, &base_field).unwrap_or_default(),
            test_piece
        )
    };

    let mut add_piece_up = |prev_piece: Piece| -> (bool, Piece) {
        let test_piece = prev_piece.clone_with_offset(0, 1);

        if piece_fits_over(test_piece, 0, &base_field).unwrap_or_default() {
            stem_positions.push(test_piece);
            
            return (true, test_piece)
        }
        (false, test_piece)
    };

    // add all pieces up from the base piece to stem_positions until blocked
    // add_piece_up pushes to stem_positions
    let mut up_piece = base_piece;
    loop {
        let (was_added, new_up_piece) = add_piece_up(up_piece);
        if !was_added { break }
        
        up_piece = new_up_piece;
    }

    // for each stem position push out left and right
    // if the position can be harddropped then return true
    // if the position can go up make another stem
    // else continue going left and right until blocked
    for stem_piece in stem_positions.iter() {        
        let mut left_piece = *stem_piece;
        while let Some((works, new_left_piece)) = test_piece_left(left_piece) {
            if works { return true }

            // if a new stem can be created do so
            let (make_stem, new_stem_piece) = test_piece_up(new_left_piece);
            if make_stem {
                if softdrop_stem_check(new_stem_piece, &base_field) {
                    return true
                }
            }

            left_piece = new_left_piece;
        }

        let mut right_piece = *stem_piece;
        while let Some((works, new_right_piece)) = test_piece_right(right_piece)
        {
            if works { return true }

            // if a new stem can be created do so
            let (make_stem, new_stem_piece) = test_piece_up(new_right_piece);
            if make_stem {
                if softdrop_stem_check(new_stem_piece, &base_field) {
                    return true
                }
            }
            
            right_piece = new_right_piece;
        }
    }

    false
}

pub fn can_harddrop(piece: Piece, field: &FieldMatrix) -> bool {
    let mut empty_space_below_piece = true;

    for (x, y) in piece_block_positions(piece).unwrap().iter() {
                // if colummn above piece has no blockage
        if field
            .iter()
            .map(|row| row[*x])
            .take(*y) // fix/test
            .any(|n| n != 0)
        {
            return false
        }
        if *y == 23 || field[*y + 1][*x] != 0 {
            empty_space_below_piece = false
        }
    }

    if empty_space_below_piece {
        return false
    }

    true
}

pub fn place_piece_on_field(piece: Piece, field: &mut FieldMatrix) {
    // does not care what is originally placed on field: overwrites anyway
    
    let piece_index = piece_type_to_fumen_index(piece.piece_type);

    for (x, y) in piece_block_positions(piece).unwrap().iter() {
        field[*y][*x] = piece_index;
    }
}

fn inbounds(x: usize, y: usize) -> bool {
    x < 10 && y < 24
}

fn signed_inbounds(x: isize, y: isize) -> bool {
    x < 10 && y < 24 && x >= 0 && y >= 0
}

fn piece_check_offset(piece_type: PieceType) -> (isize, isize) {
    // the upper left part of a piece's collision is not always a filled block
    // so if you have a point on the field and want to test it to part of a
    // piece's collision, you have to have some offset so the block selected
    // is actually overlapping with the piece's collision.
    
    match piece_type {
        PieceType::O => (0, 0),
        PieceType::I => (-2, -1),
        _ => (-1, -1),
    }
}

pub fn impossibilites(pieces: &Vec<Piece>, field: &FieldMatrix) -> Vec<Piece> {    
    let mut impossible_pieces = Vec::new();
    
    for piece in pieces.iter() {
        let mut piece_supported = false;
        for (x, y) in piece_block_positions(*piece).unwrap().iter() {
            if *y == 23 || field[*y + 1][*x] != 0 {
                piece_supported = true;
                break
            }
        }
        if !piece_supported {
            impossible_pieces.push(*piece);
        }
        // todo more impossible checks
    }

    impossible_pieces
}