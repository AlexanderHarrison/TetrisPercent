use std::convert::TryInto;
use std::cmp::min;
use urlparse::quote;

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
        matrix[n / 10][n % 10] = 
        field[n].try_into().expect("Block in fumen not allowed");
    }
        
    matrix
}
/*
pub fn matrix_to_field(matrix: FieldMatrix) -> FieldData {
    let mut field = [0; 240];
    
    for (i, v) in matrix.into_iter().flatten().enumerate() {
        field[i] = *v;
    }

    field
}*/

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
            val = data[i] + (data[i + 1] * 64) + (data[i + 2] * 4096) + (
                data[i + 3] * 262144) + (data[i + 4] * 16777216);
            i += 5;
            comment.push(ASC_TABLE.chars().nth(val % 96).unwrap());
            val = val / 96;
            comment.push(ASC_TABLE.chars().nth(val % 96).unwrap());
            val = val / 96;
            comment.push(ASC_TABLE.chars().nth(val % 96).unwrap());
            val = val / 96;
            comment.push(ASC_TABLE.chars().nth(val % 96).unwrap());
        }
        comment = comment[..comment_len].to_string();  // strip padding

        // note: fumen uses unescape to support unicode, 
        // but I'm not going to bother trying to simulate unescape.
        // handle %uxxxx
    }
    if data.len() - i > 0 {
        panic!("Data remaining after first frame parsed.");
    }
    (field_to_matrix(field), comment)
}

pub fn encode(frames: Vec<(FieldMatrix, &str)>) -> String {
    let mut data: Vec<usize> = Vec::new();
    let mut prev_comment = "";
    let mut ct_flag = 1;
    let mut prev_frame = [0; 240];
    for (field, comment) in frames.iter() {
        let mut new_frame = [0; 240];
        
        // add field from bottom->top into blank frame
        for (y, row) in field.iter().enumerate() {
            for x in 0..10 {
                new_frame[(y * 10) + x] = row[x];
            }
        }

        // fumen encoding starts here
        let mut frame = [0; 240];
        for i in 0..240 {
            frame[i] += new_frame[i] + 8 - prev_frame[i]
        }

        // simple run-length encoding for field-data
        let mut repeat_count = 0;
        for j in 0..(240 - 1) {
            repeat_count += 1;
            if frame[j] != frame[j + 1] {
                let mut val: usize = 
                    ((frame[j] * 240) + (repeat_count - 1)).try_into().unwrap();
                data.push(val % 64);
                val = val / 64;
                data.push(val % 64);
                repeat_count = 0;
            }
        }

        // output final block
        let mut val: usize =
            ((frame[240 - 1] * 240) + (repeat_count)).try_into().unwrap();
        data.push(val % 64);
        val = val / 64;
        data.push(val % 64);

        // ignore check for blank frame/field repeat here

        // piece/data output
        // I implement here comment flag + "ct" flag (Guideline colors)

        val = match *comment != prev_comment {
            true => 1,
            false => 0
        };
        val = 128 * 240 * ((val * 2) + ct_flag);
        ct_flag = 0; // should only be set on the first frame
        
        data.push(val % 64);
        val = val / 64;
        data.push(val % 64);
        val = val / 64;
        data.push(val % 64);
        
        if *comment != prev_comment {
            // quote simulates escape() in javascript
            // but output is not one-to-one (since escape is deprecated)

            let comment_str = quote(
                &comment[..min(4096, comment.chars().count())],
                b""
            ).ok().unwrap();

            let comment_len = comment_str.len();
            let mut comment_data = comment_str
                .chars()
                .map(
                    |c| ASC_TABLE.find(c).unwrap())
                .collect::<Vec<usize>>();
            
            // pad data if necessary
            if (comment_len % 4) > 0 {
                comment_data.extend(
                        [0]
                        .iter()
                        .cycle()
                        .take(4 - (comment_len % 4)));
            }
                    
            // output length of comment
            val = comment_len;
            data.push(val % 64);
            val = val / 64;
            data.push(val % 64);

            // every 4 chars becomes 5 bytes 
            
            //    (4 * 96 chars in ASCII table = 5 * 64)

            for i in (0..comment_len).step_by(4) {
                val = comment_data[i];
                val += comment_data[i + 1] * 96;
                val += comment_data[i + 2] * 9216;
                val += comment_data[i + 3] * 884736;
                data.push(val % 64);
                val = val / 64;
                data.push(val % 64);
                val = val / 64;
                data.push(val % 64);
                val = val / 64;
                data.push(val % 64);
                val = val / 64;
                data.push(val % 64);
            }
        }

        prev_frame = new_frame;
        prev_comment = comment;
    }

    let mut encode_str = "v115@".to_string();
    for (i, output_byte) in data.iter().enumerate() {
        encode_str.push(ENC_TABLE.chars().nth(*output_byte).unwrap());
        if i % 47 == 41 {
            encode_str += "?";
        }
    }

    encode_str
}