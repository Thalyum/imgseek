//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
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

#[derive(Debug, Clone, Copy)]
struct DisplayInfo {
    piece_index: usize,
    column_index: usize,
}

#[derive(Debug)]
enum DisplayEvent {
    Start(DisplayInfo),
    End(DisplayInfo),
}

impl DisplayEvent {
    fn piece_index(&self) -> usize {
        match self {
            Self::Start(e) => e.piece_index,
            Self::End(e) => e.piece_index,
        }
    }

    fn display(&self) -> String {
        match self {
            Self::Start(_) => "--Start--".to_owned(),
            Self::End(_) => "--End--".to_owned(),
        }
    }
}

impl fmt::Display for DisplayEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[derive(Debug)]
pub struct PuzzleDisplay {
    inner: Vec<DisplayEvent>,
    pieces: Vec<PuzzlePiece>,
}

impl PuzzleDisplay {
    pub fn new() -> PuzzleDisplay {
        PuzzleDisplay {
            inner: Vec::<DisplayEvent>::new(),
            pieces: Vec::<PuzzlePiece>::new(),
        }
    }

    fn get_piece_from_event(&self, event: &DisplayEvent) -> &PuzzlePiece {
        &self.pieces[event.piece_index()]
    }

    fn is_offset_lt(&self, e: &DisplayEvent, offset: usize) -> bool {
        let elem_offset: usize = match e {
            DisplayEvent::Start(_) => self.get_piece_from_event(e).start(),
            DisplayEvent::End(_) => {
                let piece = self.get_piece_from_event(e);
                piece.start() + piece.len()
            }
        };
        elem_offset > offset
    }

    pub fn add_element(&mut self, new_piece: PuzzlePiece) {
        let start_addr = new_piece.start();
        let end_addr = start_addr + new_piece.len();

        self.pieces.push(new_piece);
        let piece_index = self.pieces.len() - 1;

        let mut new_piece = DisplayInfo {
            piece_index,
            column_index: 0,
        };

        let mut start_index: Option<usize> = None;
        let mut end_index: Option<usize> = None;
        // search for where to insert the 'Start' of the puzzle piece
        for (i, event) in self.inner.iter().enumerate() {
            if self.is_offset_lt(event, start_addr) {
                start_index = Some(i);
                // search for where to insert the 'End' of the puzzle piece after the 'Start' one
                for (j, event) in self.inner[i..].iter().enumerate() {
                    if self.is_offset_lt(event, end_addr) {
                        end_index = Some(i + j);
                        break;
                    }
                }
                break;
            }
        }

        self.insert_piece(new_piece, start_index, end_index);
    }

    fn insert_piece(
        &mut self,
        new_piece: DisplayInfo,
        start_index: Option<usize>,
        end_index: Option<usize>,
    ) {
        let piece_start = DisplayEvent::Start(new_piece);
        if let Some(index) = start_index {
            self.inner.insert(index, piece_start);
        } else {
            self.inner.push(piece_start);
        }
        let piece_end = DisplayEvent::End(new_piece);
        if let Some(index) = end_index {
            self.inner.insert(index, piece_end);
        } else {
            self.inner.push(piece_end);
        }
    }

    // fn compute_width(&mut self) {
    //     let mut v: Vec<(usize, usize)> = Vec::new();
    //     let mut n_start = 0;
    //     let mut n_end = 0;
    //     for elem in self.inner.iter() {
    //         match elem {
    //             DisplayEvent::Start(_) => n_start += 1,
    //             DisplayEvent::End(_) => n_end += 1,
    //         }
    //         v.push((n_start, n_end));
    //     }

    //     assert_eq!(n_start, n_end);
    //     assert_eq!(2 * n_start, self.inner.len());

    //     self.availability = v.iter().map(|(a, b)| a - b).max().unwrap();
    // }

    // pub fn compute_all_indexes(&mut self) {
    //     self.compute_width();
    //     assert_ne!(self.availability, 0);

    //     let mut availability = vec![0; self.availability];

    //     for element in self.inner.iter_mut() {
    //         if let DisplayEvent::Start(e) = element {
    //             // Update 'Start' index
    //             if let Some(index) = availability.iter().position(|&x| x == 0) {
    //                 e.column_index = index;
    //                 availability[index] = 1;
    //             }
    //             // // Update 'End' index, located after
    //             // self.inner
    //             //     .iter()
    //             //     .position(|DisplayEvent::End(end)| end.inner == e.inner);
    //         }
    //     }
    // }

    fn display(&self) -> String {
        let mut fmt = String::new();
        for event in self.inner.iter() {
            fmt = format!("{}{}\n{}\n", fmt, event, {
                let piece = self.get_piece_from_event(event);
                piece.name()
            });
        }
        fmt
    }
}

impl fmt::Display for PuzzleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
