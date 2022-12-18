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
        self.display_create_top_border(display_w, widen, &mut display);
        self.display_create_lines(table_w, widen, &mut display);
        self.display_create_bottom_border(table_h, widen, &mut display);
        self.display_create_footer(&mut display);

        // FIXME: temp debug
        println!("{}", self.parray.array);

        format!("{}", display)
    }

    fn is_start_of_line(col_index: usize, total_width: usize) -> bool {
        col_index % (total_width - 1) == 0
    }

    fn is_end_of_line(col_index: usize, total_width: usize) -> bool {
        (col_index + 1) % (total_width - 1) == 0
    }

    fn display_create_lines(&self, table_w: usize, widen: usize, display: &mut String) {
        // create "lines" of the array
        for (n, win) in self.parray.array.windows((2, 2)).into_iter().enumerate() {
            let mut line = String::new();

            // start of line
            if Self::is_start_of_line(n, table_w) {
                line.push('│');
            }
            let root = win[[0, 0]];
            let right = win[[0, 1]];
            let under = win[[1, 0]];
            let c = if root != under { '_' } else { ' ' };
            line_push_multiple(&mut line, c, widen);
            let c = if root != right { '│' } else { ' ' };
            line.push(c);

            // end of line
            if Self::is_end_of_line(n, table_w) {
                let corner = win[[1, 1]];
                let c = if right != corner { '_' } else { ' ' };
                line_push_multiple(&mut line, c, widen);
                line.push('│');
                {
                    let line_index = n / (table_w - 1);
                    let line_offset = self.parray.offset_list[line_index];
                    Self::display_append_offset_hint(&mut line, line_offset);
                }
                line.push('\n');
            }

            display.push_str(&line);
        }
    }

    fn display_create_bottom_border(&self, table_h: usize, widen: usize, display: &mut String) {
        let last_row = self.parray.array.row(table_h - 1);
        let mut last_line = String::new();
        last_line.push('└');
        for win in last_row.windows(2) {
            last_line.push('─');
            let c = if win[0] != win[1] { '│' } else { '─' };
            line_push_multiple(&mut last_line, c, widen);
        }
        line_push_multiple(&mut last_line, '─', widen);
        last_line.push('┘');
        Self::display_append_offset_hint(&mut last_line, *self.parray.offset_list.last().unwrap());
        last_line.push('\n');
        display.push_str(&last_line);
    }

    fn display_create_footer(&self, display: &mut String) {
        for (index, piece) in self.pieces.iter().enumerate() {
            let piece_name = format!("{}: '{}'\n", index, &piece.name());
            display.push_str(&piece_name);
        }
    }

    fn display_create_top_border(&self, display_w: usize, widen: usize, display: &mut String) {
        let mut tb_border = String::new();
        tb_border.push('┌');
        for _ in 0..display_w - 1 {
            line_push_multiple(&mut tb_border, '─', widen + 1);
        }
        line_push_multiple(&mut tb_border, '─', widen);
        tb_border.push('┐');
        Self::display_append_offset_hint(&mut tb_border, self.parray.offset_list[0]);
        tb_border.push('\n');
        display.push_str(&tb_border);
    }

    fn display_append_offset_hint(str_line: &mut String, offset: usize) {
        let offset_hint = format!(" <- {:#08x}", offset);
        str_line.push_str(&offset_hint);
    }
}

fn line_push_multiple(line: &mut String, c: char, amount: usize) {
    for _ in 0..amount {
        line.push(c);
    }
}

impl fmt::Display for PuzzleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
