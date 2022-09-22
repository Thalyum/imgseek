//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
use std::fmt;
use std::rc::Rc;

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

    fn get_name(&self) -> &str {
        self.bin_name.as_str()
    }

    // TODO
    fn block_display(&self) -> String {
        let width = self.bin_name.len() + 5;
        let height = 3;

        let mut border = String::with_capacity(width * height);

        // top border
        border.push('+');
        for _ in 0..width - 3 {
            border.push('-');
        }
        border.push('+');

        // offset
        border += format!(" <- {:#010x}\n", self.bin_offset).as_str();

        // fill block name
        border.push('|');
        border.push(' ');
        border.push_str(&self.bin_name);
        border.push(' ');
        border.push('|');
        border.push('\n');

        // bottom border
        border.push('+');
        for _ in 0..width - 3 {
            border.push('-');
        }
        border.push('+');

        // end offset
        border += format!(" <- {:#010x}", self.bin_offset + self.bin_size).as_str();

        border
    }
}

impl fmt::Display for PuzzlePiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.block_display())
    }
}

impl fmt::Debug for PuzzlePiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.block_display())
    }
}

#[derive(Debug)]
enum DisplayElement {
    Start(Rc<PuzzlePiece>),
    End(Rc<PuzzlePiece>),
}

#[derive(Debug)]
pub struct PuzzleDisplay {
    inner: Vec<DisplayElement>,
}

impl PuzzleDisplay {
    pub fn new() -> PuzzleDisplay {
        PuzzleDisplay {
            inner: Vec::<DisplayElement>::new(),
        }
    }

    fn is_correct_index(e: &DisplayElement, offset: usize) -> bool {
        let elem_offset: usize = match e {
            DisplayElement::Start(x) => x.start(),
            DisplayElement::End(x) => x.start() + x.len(),
        };
        elem_offset > offset
    }

    pub fn add_element(&mut self, new_piece: PuzzlePiece) -> () {
        let start_addr = new_piece.start();
        let end_addr = start_addr + new_piece.len();

        let new_piece_s = Rc::new(new_piece);
        let new_piece_e = Rc::clone(&new_piece_s);

        let find_start = |e: &DisplayElement| PuzzleDisplay::is_correct_index(e, start_addr);
        let find_end = |e: &DisplayElement| PuzzleDisplay::is_correct_index(e, end_addr);

        if let Some(i) = self.inner.iter().position(find_start) {
            self.inner.insert(i, DisplayElement::Start(new_piece_s));
            if let Some(j) = self.inner[i..].iter().position(find_end) {
                self.inner.insert(i + j, DisplayElement::End(new_piece_e));
            } else {
                self.inner.push(DisplayElement::End(new_piece_e));
            }
        } else {
            self.inner.push(DisplayElement::Start(new_piece_s));
            self.inner.push(DisplayElement::End(new_piece_e));
        }

        ()
    }
}
