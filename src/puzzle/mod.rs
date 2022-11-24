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
}

impl fmt::Display for PuzzleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parray)
    }
}
