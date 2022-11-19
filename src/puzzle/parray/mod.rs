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

pub struct PArray {
    array: Array2<SlotStatus>,
    offset_list: Vec<usize>,
}

impl PArray {
    pub fn new(image_size: u64) -> Self {
        let shape = (2, 1);
        let array: Array2<SlotStatus> = Array::ones(shape);

        let offset_list = vec![0, image_size as usize];

        assert_eq!(offset_list.len(), array.shape()[0]);

        Self { array, offset_list }
    }

    pub fn insert_row(&mut self, new_offset: usize) -> Result<&mut Self> {
        // find index for row insertion
        let index = self.offset_list.iter().position(|&v| v >= new_offset);
        // add a new row at the end
        self.push_row()?;

        Ok(self)
    }

    fn push_row(&mut self) -> Result<&mut Self> {
        let vec = &vec![SlotStatus::Free; self.array.ncols()];
        let new_row = ArrayView::from(vec);
        self.array.push_row(new_row)?;
        Ok(self)
    }
}

impl fmt::Display for PArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x?}\n{}", self.offset_list, self.array)
    }
}
