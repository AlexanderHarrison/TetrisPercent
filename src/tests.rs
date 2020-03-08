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