//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
mod corner;
mod dynzip;
mod parray;

use crate::error::*;
use colored::Colorize;
use corner::{ClockWiseSlots, Corner};
use dynzip::DynamicZip;
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
    vertical_scale: usize,
    corner_set: [Corner; 11],
}

impl PuzzleDisplay {
    pub fn new(image_size: u64) -> PuzzleDisplay {
        let parray = PieceArray::new(image_size);

        let corner_set: [Corner; 11] = [
            Corner::new(' '),
            Corner::new('│').set_top().set_bottom(),
            Corner::new('┌').set_right().set_bottom(),
            Corner::new('┐').set_bottom().set_left(),
            Corner::new('└').set_top().set_right(),
            Corner::new('┘').set_top().set_left(),
            Corner::new('├').set_top().set_right().set_bottom(),
            Corner::new('┤').set_top().set_bottom().set_left(),
            Corner::new('┬').set_right().set_bottom().set_left(),
            Corner::new('┴').set_top().set_right().set_left(),
            Corner::new('┼')
                .set_left()
                .set_top()
                .set_right()
                .set_bottom(),
        ];

        PuzzleDisplay {
            pieces: Vec::<PuzzlePiece>::new(),
            parray,
            horizontal_scale: 4,
            vertical_scale: 2,
            corner_set,
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
        let mut display_vec = self.process_columns();
        // create each 'edge' columns
        self.insert_edges(&mut display_vec);
        // create display String
        let mut display = self.display_create(display_vec);
        // add footer
        self.display_create_footer(&mut display);
        format!("{}", display)
    }

    fn process_columns(&self) -> Vec<Vec<String>> {
        let mut display_vec = Vec::<Vec<String>>::new();
        // create each 'filled' columns
        for col in (&self.parray.array).columns().into_iter() {
            let mut display_col = Vec::<String>::new();
            // begin by top border
            display_col.push("─".to_string());
            // process the column by windows. Each row implies two new strings:
            // - one for the cell being processed
            // - one for the cell transition.
            for win in col.windows(2) {
                // process cell
                let cell = win[0];
                display_col.push(slot_to_colored(cell));
                // process cell transition
                let n_cell = win[1];
                display_col.push(if cell == n_cell {
                    slot_to_colored(cell)
                } else {
                    "─".to_string()
                });
            }
            // process last cell
            if let Some(&cell) = col.last() {
                display_col.push(slot_to_colored(cell));
            }
            // end by bottom border
            display_col.push("─".to_string());
            // and finish the column !
            display_vec.push(display_col);
        }
        display_vec
    }

    fn insert_edges(&self, display_vec: &mut Vec<Vec<String>>) {
        let mut col_iter = (&self.parray.array)
            .columns()
            .into_iter()
            .enumerate()
            .peekable();
        while let Some((n, column)) = col_iter.next() {
            // n is the number of 'edge' columns that we have processed
            if let Some((_, n_column)) = col_iter.peek() {
                let mut display_col = Vec::<String>::new();
                // begin by top border
                if column.first().unwrap().is_used() || n_column.first().unwrap().is_used() {
                    display_col.push("┬".to_string());
                } else {
                    display_col.push("─".to_string());
                }
                for x in column
                    .windows(2)
                    .into_iter()
                    .zip(n_column.windows(2).into_iter())
                {
                    let cwslots: ClockWiseSlots = x.try_into().unwrap();
                    let [cell_tl, cell_tr, _, _] = cwslots.inner;
                    // process cell vertical edge
                    display_col.push(if cell_tl.is_used() || cell_tr.is_used() {
                        "│".to_string()
                    } else {
                        " ".to_string()
                    });
                    // process cell transition edge
                    let position = self
                        .corner_set
                        .iter()
                        .position(|c| c.is_usable_for(cwslots))
                        .unwrap();
                    let corner = &self.corner_set[position];
                    display_col.push(String::from(corner.c));
                }
                // process last cell & bottom border
                if column.last().unwrap().is_used() || n_column.last().unwrap().is_used() {
                    display_col.push("│".to_string());
                    display_col.push("┴".to_string());
                } else {
                    display_col.push(" ".to_string());
                    display_col.push("─".to_string());
                }
                // and finish the column !
                display_vec.insert(2 * n + 1, display_col);
            }
        }
    }

    fn display_create(&self, display_vec: Vec<Vec<String>>) -> String {
        // create display, line by line, we need to work by rows this time.
        let mut display = String::new();
        // add the first & last characters manually
        // as they correspond to the left & right border of the table.
        let last_idx = display_vec.first().unwrap().len() - 1;
        let iterators: Vec<_> = display_vec.into_iter().map(|col| col.into_iter()).collect();
        let dz = DynamicZip { iterators };
        for (n, row) in dz.enumerate() {
            let mut line = String::new();
            // begin by left border
            if n == 0 {
                line.push('┌');
            } else if n == last_idx {
                line.push('└');
            } else if row.first().unwrap() == "─" {
                line.push('├');
            } else {
                line.push('│');
            }
            // add the row content
            for (n, s) in row.iter().enumerate() {
                if n % 2 == 0 {
                    for _ in 0..self.horizontal_scale {
                        line.push_str(&s);
                    }
                } else {
                    line.push_str(&s);
                }
            }
            // finish by right border
            if n == 0 {
                line.push('┐');
            } else if n == last_idx {
                line.push('┘');
            } else if row.last().unwrap() == "─" {
                line.push('┤');
            } else {
                line.push('│');
            }
            if n % 2 == 0 {
                line.push_str(&format!(" <-- {:#08x}", self.parray.offset_list[n / 2]));
            }
            line.push('\n');
            if n % 2 == 1 {
                for _ in 0..self.vertical_scale {
                    display.push_str(&line);
                }
            } else {
                display.push_str(&line);
            }
        }
        display
    }

    fn display_create_footer(&self, display: &mut String) {
        for (index, piece) in self.pieces.iter().enumerate() {
            let color = COLOR_LIST[index % COLOR_LIST.len()];
            let index_colored = index.to_string().color("black").on_color(color);
            let piece_name = format!("{}: '{}'\n", index_colored.to_string(), &piece.name());
            display.push_str(&piece_name);
        }
    }
}

fn slot_to_colored(cell: SlotStatus) -> String {
    if cell.is_used() {
        cell.try_into_used()
            .and_then(|index| Ok(COLOR_LIST[index % COLOR_LIST.len()]))
            .and_then(|color| Ok(" ".on_color(color)))
            .unwrap()
            .to_string()
    } else {
        " ".to_string()
    }
}

impl fmt::Display for PuzzleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
