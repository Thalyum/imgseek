//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
mod parray;

use crate::error::*;
use colored::Colorize;
use parray::{slot::SlotStatus, PieceArray};
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

const COLOR_LIST: [&str; 7] = ["red", "green", "yellow", "blue", "magenta", "cyan", "white"];

#[derive(Debug)]
pub struct PuzzleDisplay {
    pieces: Vec<PuzzlePiece>,
    parray: PieceArray,
    horizontal_scale: usize,
    // TODO: add height multiplier
    // vertical_scale: usize,
}

impl PuzzleDisplay {
    pub fn new(image_size: u64) -> PuzzleDisplay {
        let parray = PieceArray::new(image_size);

        PuzzleDisplay {
            pieces: Vec::<PuzzlePiece>::new(),
            parray,
            horizontal_scale: 4,
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
        // let (term_w, term_h) = term_size::dimensions().unwrap();
        // let table_w = self.parray.array.ncols();
        // let table_h = self.parray.array.nrows();

        // let display_w = std::cmp::min(term_w, table_w);
        // let display_h = std::cmp::min(term_h, table_h);

        let mut display = String::new();
        // handle top of table
        self.display_create_top_border(&mut display);
        // handle every line in between
        self.display_create_lines(&mut display);
        // handle bottom of table
        self.display_create_bottom_border(&mut display);
        // create footer images summary
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

    fn display_create_lines(&self, display: &mut String) {
        let width = self.parray.array.ncols();
        for (n, win) in self
            .parray
            .array
            .windows((2, 2))
            .into_iter()
            .enumerate()
            .skip(width - 1)
        {
            let mut line = String::new();

            // start of line
            if Self::is_start_of_line(n, width) {
                line.push('│');
            }

            let root = win[[0, 0]];
            let right = win[[0, 1]];
            let under = win[[1, 0]];
            let corner = win[[1, 1]];

            self.fill_column(root, under, &mut line);
            let c = if root != right { '│' } else { ' ' };
            line.push(c);

            // end of line
            if Self::is_end_of_line(n, width) {
                self.fill_column(right, corner, &mut line);
                line.push('│');
                {
                    let line_index = n / (width - 1);
                    let line_offset = self.parray.offset_list[line_index];
                    Self::display_append_offset_hint(&mut line, line_offset);
                }
                line.push('\n');
            }

            display.push_str(&line);
        }
    }

    fn display_create_bottom_border(&self, display: &mut String) {
        let widen = self.horizontal_scale;
        let last_row = self.parray.array.rows().into_iter().last().unwrap();
        let mut last_line = String::new();
        last_line.push('└');
        for win in last_row.windows(2) {
            let current = win[0];
            let right = win[1];

            line_push_multiple(&mut last_line, '─', self.horizontal_scale);
            let c = if current != right { '┴' } else { '─' };
            last_line.push(c);
        }
        line_push_multiple(&mut last_line, '─', widen);
        last_line.push('┘');
        Self::display_append_offset_hint(&mut last_line, *self.parray.offset_list.last().unwrap());
        last_line.push('\n');
        display.push_str(&last_line);
    }

    fn display_create_footer(&self, display: &mut String) {
        for (index, piece) in self.pieces.iter().enumerate() {
            let color = COLOR_LIST[index % COLOR_LIST.len()];
            let index_colored = index.to_string().color("black").on_color(color);
            let piece_name = format!("{}: '{}'\n", index_colored.to_string(), &piece.name());
            display.push_str(&piece_name);
        }
    }

    fn display_create_top_border(&self, display: &mut String) {
        let widen = self.horizontal_scale;
        let first_row = self.parray.array.row(0);
        let mut first_line = String::new();
        first_line.push('┌');
        for win in first_row.windows(2) {
            let current = win[0];
            let right = win[1];

            line_push_multiple(&mut first_line, '─', self.horizontal_scale);
            let c = if current != right { '┬' } else { '─' };
            first_line.push(c);
        }
        line_push_multiple(&mut first_line, '─', widen);
        first_line.push('┐');
        Self::display_append_offset_hint(&mut first_line, self.parray.offset_list[0]);
        first_line.push('\n');
        display.push_str(&first_line);
    }

    fn display_append_offset_hint(str_line: &mut String, offset: usize) {
        let offset_hint = format!(" <- {:#08x}", offset);
        str_line.push_str(&offset_hint);
    }

    fn fill_column(&self, upper: SlotStatus, lower: SlotStatus, line: &mut String) {
        let widen = self.horizontal_scale;
        let bg_color = upper
            .try_into_used()
            .and_then(|index| Ok(COLOR_LIST[index % COLOR_LIST.len()]))
            .unwrap_or("black");
        let s = if upper != lower { "_" } else { " " }.on_color(bg_color);
        line_push_str_multiple(line, s.to_string(), widen);
    }
}

fn line_push_multiple(line: &mut String, c: char, amount: usize) {
    for _ in 0..amount {
        line.push(c);
    }
}

fn line_push_str_multiple(line: &mut String, string: String, amount: usize) {
    for _ in 0..amount {
        line.push_str(&string);
    }
}

impl fmt::Display for PuzzleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
