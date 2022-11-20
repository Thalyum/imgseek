//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
mod slot;

use crate::error::*;
use ndarray::{Array, Array2, ArrayView};
use slot::SlotStatus;
use std::fmt;

#[derive(Debug)]
pub struct PieceArray {
    array: Array2<SlotStatus>,
    offset_list: Vec<usize>,
}

impl PieceArray {
    pub fn new(image_size: u64) -> Self {
        let shape = (2, 1);
        let array: Array2<SlotStatus> = Array::zeros(shape);

        let offset_list = vec![0, image_size as usize];

        // replace assert by unit tests
        assert_eq!(offset_list.len(), array.shape()[0]);

        Self { array, offset_list }
    }

    pub fn add_piece(&mut self, piece_index: usize, start: usize, end: usize) -> Result<()> {
        let (start_index, end_index) = self.find_insert_index(start, end)?;

        let col_index = match self.find_empty_column(start_index, end_index) {
            Ok(col_index) => col_index,
            // No empty place found, create a new column to hold our piece
            Err(Error::FreeColNotFound) => {
                self.push_col()?;
                self.array.ncols() - 1
            }
            Err(e) => return Err(e),
        };

        let mut new_col = self.array.column_mut(col_index);
        for i in start_index..=end_index {
            new_col[i] = SlotStatus::Used(piece_index);
        }

        // TODO: replace assert by unit tests
        Ok(assert_eq!(self.array.nrows(), self.offset_list.len()))
    }

    fn find_empty_column(&self, start_index: usize, end_index: usize) -> Result<usize> {
        // size of the piece to add
        let length = end_index - start_index + 1;
        // search for enough 'Free' slots among the available columns
        match self.array.columns().into_iter().position(|column| {
            column
                .iter()
                .skip(start_index)
                .take(length)
                .all(|&slot| slot.is_free())
        }) {
            Some(col_index) => Ok(col_index),
            None => Err(Error::FreeColNotFound),
        }
    }

    fn find_insert_index(&mut self, start: usize, end: usize) -> Result<(usize, usize)> {
        // find index for row insertion
        let mut start_index: Option<usize> = None;
        let mut end_index: Option<usize> = None;
        for (i, &offset) in self.offset_list.iter().enumerate() {
            if offset >= start {
                start_index = Some(i);
                // search for where to insert the 'End' of the puzzle piece after the 'Start' one
                for (j, &offset) in self.offset_list[i..].iter().enumerate() {
                    if offset >= end {
                        end_index = Some(i + j + 1);
                        break;
                    }
                }
                break;
            }
        }

        let start_index = self.insert_start_to_index(start_index, start)?;
        let end_index = self.insert_end_to_index(end_index, end)?;
        Ok((start_index, end_index))
    }

    fn insert_start_to_index(&mut self, index: Option<usize>, start: usize) -> Result<usize> {
        let index_of_insertion = if let Some(index) = index {
            if index > 0 {
                // copy previous row
                let prev_row = self.array.row(index - 1).to_vec();
                self.insert_row(index, Some(prev_row))?;
            } else {
                self.insert_row(index, None)?;
            };
            self.offset_list.insert(index, start);
            index
        } else {
            self.push_row(None)?;
            self.offset_list.push(start);
            self.array.nrows() - 1
        };
        Ok(index_of_insertion)
    }

    fn insert_end_to_index(&mut self, index: Option<usize>, end: usize) -> Result<usize> {
        let index_of_insertion = if let Some(index) = index {
            if index < self.array.nrows() - 1 {
                // copy next row
                let prev_row = self.array.row(index + 1).to_vec();
                self.insert_row(index, Some(prev_row))?;
            } else {
                self.insert_row(index, None)?;
            };
            self.offset_list.insert(index, end);
            index
        } else {
            self.push_row(None)?;
            self.offset_list.push(end);
            self.array.nrows() - 1
        };
        Ok(index_of_insertion)
    }

    fn insert_row(&mut self, index: usize, row: Option<Vec<SlotStatus>>) -> Result<()> {
        // add a new row (bottom)
        self.push_row(row)?;
        // generate a row rotation matrix
        let rot = self.gen_rotate1_back_matrix(index)?;
        // rotate the matrix' rows to put the new row at the desired index
        self.array = rot.dot(&self.array);
        Ok(())
    }

    // add a new row at the bottom of the array
    fn push_row(&mut self, row: Option<Vec<SlotStatus>>) -> Result<()> {
        let vec = match row {
            Some(vec) => vec,
            None => vec![SlotStatus::Free; self.array.ncols()],
        };
        let new_row = ArrayView::from(&vec);
        Ok(self.array.push_row(new_row)?)
    }

    // add a new column at the right of the array
    fn push_col(&mut self) -> Result<()> {
        let vec = vec![SlotStatus::Free; self.array.nrows()];
        let new_col = ArrayView::from(&vec);
        Ok(self.array.push_column(new_col)?)
    }

    // generate an Identity Matrix (2D array) with same dimensions as self.array
    // e.g for index = 2
    // [[▓, ░, ░, ░, ░, ░, ░],
    //  [░, ▓, ░, ░, ░, ░, ░],
    //  [░, ░, ░, ░, ░, ░, ▓],
    //  [░, ░, ▓, ░, ░, ░, ░],
    //  [░, ░, ░, ▓, ░, ░, ░],
    //  [░, ░, ░, ░, ▓, ░, ░],
    //  [░, ░, ░, ░, ░, ▓, ░]]
    fn gen_rotate1_back_matrix(&self, index: usize) -> Result<Array2<SlotStatus>> {
        let n_rows = self.array.nrows();
        let shape = (n_rows, n_rows);

        let mut vec: Vec<SlotStatus> = vec![];
        for i in 0..n_rows {
            let mut r = vec![SlotStatus::Free; n_rows];
            let n = if i < index {
                i
            } else if i == index {
                n_rows - 1
            } else {
                i - 1
            };
            r[n] = SlotStatus::Identity;
            vec.append(&mut r);
        }

        let id: Array2<SlotStatus> = Array2::from_shape_vec(shape, vec)?;
        Ok(id)
    }
}

impl fmt::Display for PieceArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x?}\n{}", self.offset_list, self.array)
    }
}
