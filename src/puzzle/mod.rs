//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
mod parray;

use crate::error::*;
use colored::Colorize;
use ndarray::{ArrayBase, Ix1, Ix2, ViewRepr};
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

#[derive(Debug)]
struct Corner {
    // 'corner' unicode character to draw
    c: char,
    // clockwise (starting from top) list of edges that characterize the corner
    split: [bool; 4],
}

impl Corner {
    fn new(c: char) -> Self {
        Self {
            c,
            split: [false, false, false, false],
        }
    }

    fn set_top(mut self) -> Self {
        self.split[0] = true;
        self
    }

    fn set_right(mut self) -> Self {
        self.split[1] = true;
        self
    }

    fn set_bottom(mut self) -> Self {
        self.split[2] = true;
        self
    }

    fn set_left(mut self) -> Self {
        self.split[3] = true;
        self
    }

    /// Check if the Corner can be used in this case, based on the list of SlotStatus around the corner.
    fn is_usable_for(&self, slots: ClockWiseSlots) -> bool {
        let template: [bool; 4] = [
            slots.inner[0] != slots.inner[1],
            slots.inner[1] != slots.inner[2],
            slots.inner[2] != slots.inner[3],
            slots.inner[3] != slots.inner[0],
        ];
        self.split.iter().zip(template.iter()).all(|(a, b)| a == b)
    }
}

#[derive(Clone, Copy)]
struct ClockWiseSlots {
    inner: [SlotStatus; 4],
}

impl TryFrom<ArrayBase<ViewRepr<&SlotStatus>, Ix2>> for ClockWiseSlots {
    type Error = Error;

    fn try_from(value: ArrayBase<ViewRepr<&SlotStatus>, Ix2>) -> Result<Self> {
        if value.dim() != (2, 2) {
            Err(Error::BadShape)
        } else {
            Ok(Self {
                inner: [value[[0, 0]], value[[0, 1]], value[[1, 1]], value[[1, 0]]],
            })
        }
    }
}

type Custom<'a> = ArrayBase<ViewRepr<&'a SlotStatus>, Ix1>;

impl TryFrom<(Custom<'_>, Custom<'_>)> for ClockWiseSlots {
    type Error = Error;

    fn try_from(value: (Custom<'_>, Custom<'_>)) -> Result<Self> {
        if value.0.dim() != (2) || value.1.dim() != (2) {
            Err(Error::BadShape)
        } else {
            Ok(Self {
                inner: [value.0[[0]], value.1[[0]], value.1[[1]], value.0[[1]]],
            })
        }
    }
}

// https://stackoverflow.com/questions/54756166/how-do-i-efficiently-iterate-through-a-vecvect-row-by-row
struct DynamicZip<I>
where
    I: Iterator,
{
    iterators: Vec<I>,
}

impl<I, T> Iterator for DynamicZip<I>
where
    I: Iterator<Item = T>,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Vec<T>> = self.iterators.iter_mut().map(|iter| iter.next()).collect();
        output
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
        // FIXME: temp debug
        println!("{}", self.parray.array);
        println!("{:#x?}", self.parray.offset_list);

        // let (term_w, term_h) = term_size::dimensions().unwrap();
        // let table_w = self.parray.array.ncols();
        // let table_h = self.parray.array.nrows();

        // let display_w = std::cmp::min(term_w, table_w);
        // let display_h = std::cmp::min(term_h, table_h);
        let array = &self.parray.array;

        let mut display_vec = Vec::<Vec<String>>::new();
        // create each 'filled' columns
        for col in array.columns().into_iter() {
            let mut display_col = Vec::<String>::new();
            // begin by top border
            display_col.push("─".to_string());
            // process the column by windows. Each row implies two new strings:
            // - one for the cell being processed
            // - one for the cell transition.
            for win in col.windows(2) {
                // process cell
                let cell = win[0];
                display_col.push(if cell.is_used() {
                    cell.try_into_used()
                        .and_then(|index| Ok(COLOR_LIST[index % COLOR_LIST.len()]))
                        .and_then(|color| Ok(" ".on_color(color)))
                        .unwrap()
                        .to_string()
                } else {
                    " ".to_string()
                });
                // process cell transition
                let n_cell = win[1];
                display_col.push(if cell == n_cell {
                    if cell.is_used() {
                        cell.try_into_used()
                            .and_then(|index| Ok(COLOR_LIST[index % COLOR_LIST.len()]))
                            .and_then(|color| Ok(" ".on_color(color)))
                            .unwrap()
                            .to_string()
                    } else {
                        " ".to_string()
                    }
                } else {
                    "─".to_string()
                });
            }
            // process last cell
            let cell = col.last().unwrap();
            display_col.push(if cell.is_used() {
                cell.try_into_used()
                    .and_then(|index| Ok(COLOR_LIST[index % COLOR_LIST.len()]))
                    .and_then(|color| Ok(" ".on_color(color)))
                    .unwrap()
                    .to_string()
            } else {
                " ".to_string()
            });
            // end by bottom border
            display_col.push("─".to_string());
            // and finish the column !
            display_vec.push(display_col);
        }
        // create each 'edge' columns
        let mut col_iter = array.columns().into_iter().enumerate().peekable();
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
        // create display, line by line, we need to work by rows this time.
        let mut display = String::new();
        // add the first & last characters manually, as they correspond to the left & right border of the table.
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
            line.push('\n');
            display.push_str(&line);
        }

        self.display_create_footer(&mut display);
        format!("{}", display)
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

impl fmt::Display for PuzzleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
