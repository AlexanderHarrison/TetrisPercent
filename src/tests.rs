use crate::piece::*;

#[test]
fn test_piece_block_positions() {
    let mut piece = Piece {
        piece_type: PieceType::I,
        position: (1, 1),
        rotation: Rotation::Normal,
    };

    assert!(piece_block_positions(piece) != None);

    piece.position = (0, 0);
    assert!(piece_block_positions(piece) != None);

    piece.position = (9, 0);
    assert!(piece_block_positions(piece) == None);

    piece.position = (7, 16);
    piece.rotation = Rotation::Right;
    assert!(piece_block_positions(piece) 
        == Some(vec![(9, 16), (9, 17), (9, 18), (9, 19)]));
}

#[test]
fn test_can_harddrop() {
    let mut piece = Piece {
        piece_type: PieceType::I,
        position: (1, 1),
        rotation: Rotation::Normal,
    };

    let field = [[0; 10]; 24];
    
    assert!(can_harddrop(piece, &field) == false);

    piece.position = (0, 22);
    assert!(can_harddrop(piece, &field) == true);

    piece.position = (6, 22);
    assert!(can_harddrop(piece, &field) == true);
}

#[test]
fn test_unused_points() {
    let piece = Piece {
        piece_type: PieceType::I,
        position: (1, 1),
        rotation: Rotation::Normal,
    };

    let mut pieces = Vec::new();
    pieces.push(piece);

    let mut field = [[0; 10]; 24];
    assert!(find_unused_points(&pieces, &field).len() == 0);
    
    field[1][5] = 1;
    assert!(find_unused_points(&pieces, &field).len() == 0);

    field[0][0] = 2;
    let points = find_unused_points(&pieces, &field);
    assert!(points.len() == 1);
    assert!(points[0] == (0, 0));
    field[0][0] = 0;

    field[1][2] = 3;
    field[2][2] = 3;
    field[3][2] = 3;
    field[4][2] = 3;

    assert!(find_unused_points(&pieces, &field).len() == 0);
}