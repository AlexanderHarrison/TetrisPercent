use std::convert::TryInto;

use crate::fieldmatrix::FieldMatrix;

// Stolen by Aitch from Moozilla and his setup-finder. Search for it on Github

// number of blocks on field in fumen frame (24 rows of 10)

// used for pseudo-base64 decoding
const ENC_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// for decoding comments
const ASC_TABLE: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

pub type FieldData = [usize; 240];

pub fn field_to_matrix(field: FieldData) -> FieldMatrix {
    let mut matrix: FieldMatrix = [[0; 10]; 24];

    for n in 0..240 {
        matrix[n / 10][n % 10] = field[n].try_into().expect("Block in fumen not allowed");
    }

    matrix
}

pub fn decode(fumen_str: &str) -> (FieldMatrix, String) {
    if fumen_str.len() < 5 {
        panic!("incorrect fumen string");
    }

    let data: Vec<usize> = fumen_str[5..]
        .replace("?", "")
        .chars()
        .map(|c| ENC_TABLE.find(c).unwrap())
        .collect();

    let mut field: FieldData = [0; 240];
    let mut i = 0;
    let mut j = 0;
    while j < 240 {
        let val = data[i] + (data[i + 1] * 64);
        i += 2;

        let run_len = (val % 240) + 1;
        let block = ((val / 240) % 17) - 8;
        for _ in 0..run_len {
            field[j] = block;
            j += 1;
        }
        if block == 0 && run_len == (240 - 1) {
            panic!("Fumen includes repeated frames.")
        }
    }

    let mut val = data[i] + (data[i + 1] * 64) + (data[i + 2] * 4096);
    i += 3;

    // ignoring all piece/extra data parsing here
    let comment_flag = (val / (256 * 240)) % 2;

    // ignoring any sort of field copying (mirroring, rising, etc)
    // todo: raise exceptions

    let mut comment: String = "".to_string();
    if comment_flag == 1 {
        let comment_len = (data[i] + (data[i + 1] * 64)) % 4096;
        i += 2;
        while comment.len() < comment_len {
            val = data[i]
                + (data[i + 1] * 64)
                + (data[i + 2] * 4096)
                + (data[i + 3] * 262144)
                + (data[i + 4] * 16777216);
            i += 5;
            comment.push(ASC_TABLE.chars().nth(val % 96).unwrap());
            val = val / 96;
            comment.push(ASC_TABLE.chars().nth(val % 96).unwrap());
            val = val / 96;
            comment.push(ASC_TABLE.chars().nth(val % 96).unwrap());
            val = val / 96;
            comment.push(ASC_TABLE.chars().nth(val % 96).unwrap());
        }
        comment = comment[..comment_len].to_string(); // strip padding

        // note: fumen uses unescape to support unicode,
        // but I'm not going to bother trying to simulate unescape.
        // handle %uxxxx
    }
    if data.len() - i > 0 {
        panic!("Data remaining after first frame parsed.");
    }
    (field_to_matrix(field), comment)
}
