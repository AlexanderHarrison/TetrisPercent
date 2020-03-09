use std::ops::{Index, IndexMut};
use lazy_static::lazy_static;
use crate::piece::Rotation;

#[derive(Debug, Clone)]
pub struct PieceCollision {
    //clockwise; Normal -> Right -> Double -> Left
    data: [[u8; 16]; 4],
    cur_rotation: Rotation,
    pub size: usize,
}

impl PieceCollision {
    pub fn new_3(array: [[u8; 3]; 3]) -> PieceCollision {
        let mut flat_array = [0; 16];
        for (i, v) in array.iter().flatten().enumerate() {
            flat_array[i] = *v;
        }

        let data = PieceCollision::add_rotations(flat_array, 3);

        PieceCollision {
            data,
            cur_rotation: Rotation::Normal,
            size: 3
        }
    }

    pub fn new_2(array: [[u8; 2]; 2]) -> PieceCollision {
        let mut flat_array = [0; 16];
        for (i, v) in array.iter().flatten().enumerate() {
            flat_array[i] = *v;
        }

        let data = PieceCollision::add_rotations(flat_array, 2);

        PieceCollision {
            data,
            cur_rotation: Rotation::Normal,
            size: 2
        }
    }

    pub fn new_4(array: [[u8; 4]; 4]) -> PieceCollision {
        let mut flat_array = [0; 16];
        for (i, v) in array.iter().flatten().enumerate() {
            flat_array[i] = *v;
        }

        let data = PieceCollision::add_rotations(flat_array, 4);

        PieceCollision {
            data,
            cur_rotation: Rotation::Normal,
            size: 4
        }
    }

    pub fn set_rotation(&mut self, rot: Rotation) {
        self.cur_rotation = rot;
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = &u8> {
        self.data[self.rotation_i()][..self.size.pow(2)].iter()
    }

    fn rotation_i(&self) -> usize {
        match self.cur_rotation {
            Rotation::Normal => 0,
            Rotation::Right => 1,
            Rotation::Double => 2,
            Rotation::Left => 3,
        }
    }

    fn add_rotations(col: [u8; 16], size: usize) -> [[u8; 16]; 4] {
        let mut data = [[0; 16]; 4];
        let col_r = rotate(col.clone(), size, Rotation::Right);

        //clockwise; Normal -> Right -> Double -> Left
        data[0] = col;
        data[1] = col_r;
        data[2] = rotate(col.clone(), size, Rotation::Double);
        data[3] = rotate(col_r.clone(), size, Rotation::Double);

        data
    }
}

impl Index<(usize, usize)> for PieceCollision {    
    type Output = u8;
    
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        let new_index = self.size * y + x;
        
        if new_index > self.size.pow(2) - 1 {
            panic!("index out of bounds");
        }
        &self.data[self.rotation_i()][new_index]
    }
}

impl IndexMut<(usize, usize)> for PieceCollision {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let new_index = self.size * y + x;
        
        if new_index > self.size.pow(2) - 1 {
            panic!("index out of bounds");
        }
        &mut self.data[self.rotation_i()][new_index]
    }
}

lazy_static! {
    pub static ref S: PieceCollision = PieceCollision::new_3([
        [0, 1, 1], 
        [1, 1, 0], 
        [0, 0, 0]]);

    pub static ref Z: PieceCollision = PieceCollision::new_3([
        [1, 1, 0], 
        [0, 1, 1], 
        [0, 0, 0]]);

    pub static ref L: PieceCollision = PieceCollision::new_3([
        [0, 0, 1], 
        [1, 1, 1], 
        [0, 0, 0]]);

    pub static ref J: PieceCollision = PieceCollision::new_3([
        [1, 0, 0], 
        [1, 1, 1], 
        [0, 0, 0]]);

    pub static ref T: PieceCollision = PieceCollision::new_3([
        [0, 1, 0], 
        [1, 1, 1], 
        [0, 0, 0]]);

    pub static ref O: PieceCollision = PieceCollision::new_2([
        [1, 1],
        [1, 1]]);
    pub static ref I: PieceCollision = PieceCollision::new_4([
        [0, 0, 0, 0], 
        [1, 1, 1, 1], 
        [0, 0, 0, 0],
        [0, 0, 0, 0]]);
}

fn rotate(col: [u8; 16], size: usize,  rotation: Rotation) -> [u8; 16] {
    let mut new_data = [0; 16];
    match rotation {
        Rotation::Normal => col,
        Rotation::Double => {
            for (i, n) in col[..size.pow(2)].iter().rev().enumerate() {
                new_data[i] = *n;
            }
            new_data
        },
        Rotation::Left => {
            col[..size.pow(2)]
                .iter()
                .enumerate() 
                .filter(|(_, n)| **n == 1)
                .for_each(|(i, _)| {
                    let (x, y) = (i % 3, i / 3);
                    let nx = y;
                    let ny = size - 1 - x;
                    new_data[ny * size + nx] = 1;
                });
            new_data
        },
        Rotation::Right => {
            col[..size.pow(2)]
                .iter()
                .enumerate() 
                .filter(|(_, n)| **n == 1)
                .for_each(|(i, _)| {
                    let (x, y) = (i % size, i / size);
                    let nx = size - 1 - y;
                    let ny = x;
                    new_data[ny * size + nx] = 1;
                });
            new_data
        }
    }
}