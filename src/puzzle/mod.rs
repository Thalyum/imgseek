//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
mod parray;

use crate::error::*;
use parray::PieceArray;
use std::fmt;
use term_size;

#[derive(Debug, Clone)]
pub struct PuzzlePiece {
    bin_name: String,
    bin_size: usize,
    bin_offset: usize,
}

impl PuzzlePiece {
    pub fn new(bin_name: String, bin_size: usize, bin_offset: usize) -> Self {
        PuzzlePiece {
            bin_name,
            bin_size,
            bin_offset,
        }
    }

    fn start(&self) -> usize {
        self.bin_offset
    }

    fn len(&self) -> usize {
        self.bin_size
    }

    fn name(&self) -> &str {
        self.bin_name.as_str()
    }
}

#[derive(Debug)]
pub struct PuzzleDisplay {
    pieces: Vec<PuzzlePiece>,
    parray: PieceArray,
}

impl PuzzleDisplay {
    pub fn new(image_size: u64) -> PuzzleDisplay {
        let parray = PieceArray::new(image_size);

        PuzzleDisplay {
            pieces: Vec::<PuzzlePiece>::new(),
            parray,
        }
    }

    pub fn add_element(&mut self, new_piece: PuzzlePiece) -> Result<()> {
        let start_addr = new_piece.start();
        let end_addr = start_addr + new_piece.len();

        self.pieces.push(new_piece);
        let piece_index = self.pieces.len() - 1;

        Ok(self.parray.add_piece(piece_index, start_addr, end_addr)?)
    }

    pub fn display(&self) -> String {
        let (term_w, term_h) = term_size::dimensions().unwrap();
        let table_w = self.parray.array.ncols();
        let table_h = self.parray.array.nrows();

        let display_w = std::cmp::min(term_w, table_w);
        let display_h = std::cmp::min(term_h, table_h);

        let widen: usize = 4;
        // TODO: add height multiplier
        // let highten: usize = 4;

        let mut display = String::new();
        // create top & bottom border
        let mut tb_border = String::new();
        tb_border.push('┌');
        // for _ in 0..widen - 1 {
        //     tb_border.push('─');
        // }
        for _ in 0..display_w - 1 {
            for _ in 0..widen + 1 {
                tb_border.push('─');
            }
        }
        for _ in 0..widen {
            tb_border.push('─');
        }
        tb_border.push('┐');
        tb_border.push('\n');
        display.push_str(&tb_border);

        // create "lines" of the array
        for (n, win) in self.parray.array.windows((2, 2)).into_iter().enumerate() {
            let mut line = String::new();

            // start of line
            if n % (table_w - 1) == 0 {
                line.push('│');
            }
            let root = win[[0, 0]];
            let right = win[[0, 1]];
            let under = win[[1, 0]];
            let c = if root != under { '_' } else { ' ' };
            for _ in 0..widen {
                line.push(c);
            }
            let c = if root != right { '│' } else { ' ' };
            line.push(c);

            // end of line
            if (n + 1) % (table_w - 1) == 0 {
                let corner = win[[1, 1]];
                let c = if right != corner { '_' } else { ' ' };
                for _ in 0..widen {
                    line.push(c);
                }
                line.push('│');
                line.push('\n');
            }

            display.push_str(&line);
        }

        let last_row = self.parray.array.row(table_h - 1);
        let mut last_line = String::new();
        last_line.push('└');
        for win in last_row.windows(2) {
            last_line.push('─');
            let c = if win[0] != win[1] { '│' } else { '─' };
            for _ in 0..widen {
                last_line.push(c);
            }
        }
        for _ in 0..widen {
            last_line.push('─');
        }
        last_line.push('┘');
        last_line.push('\n');
        display.push_str(&last_line);

        for (index, piece) in self.pieces.iter().enumerate() {
            let piece_name = format!("{}: '{}'\n", index, &piece.name());
            display.push_str(&piece_name);
        }

        // FIXME: temp debug
        println!("{}", self.parray.array);

        format!("{}", display)
    }
}

impl fmt::Display for PuzzleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
