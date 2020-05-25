pub type FieldMatrix = [[u8; 10]; 24];

// maybe use if I need optimizations
/*use std::ops::{Index, IndexMut};

struct Row ([u8; 10]);

impl Index<usize> for Row {
    type Output = u8;

    fn index(&self, n: usize) -> &u8 {
        self.0[n];
    }
}

pub struct FieldMatrix ([Row; 24]);

impl Index<usize> for FieldMatrix {
    type Output = Row;

    fn index(&self, n: usize) -> &Row {
        &self.0[n]
    }
}

impl IndexMut<usize> for FieldMatrix {
    fn index_mut(&mut self, n: usize) -> &mut Row {
        &mut self.0[n]
    }
}*/
