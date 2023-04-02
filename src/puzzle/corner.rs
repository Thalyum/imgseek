//
// This file is part of imgseek
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
use crate::error::*;
use crate::puzzle::parray::slot::SlotStatus;
use ndarray::{ArrayBase, Ix1, Ix2, ViewRepr};

#[derive(Debug)]
pub struct Corner {
    // 'corner' unicode character to draw
    pub c: char,
    // clockwise (starting from top) list of edges that characterize the corner
    split: [bool; 4],
}

impl Corner {
    pub fn new(c: char) -> Self {
        Self {
            c,
            split: [false, false, false, false],
        }
    }

    pub fn set_top(mut self) -> Self {
        self.split[0] = true;
        self
    }

    pub fn set_right(mut self) -> Self {
        self.split[1] = true;
        self
    }

    pub fn set_bottom(mut self) -> Self {
        self.split[2] = true;
        self
    }

    pub fn set_left(mut self) -> Self {
        self.split[3] = true;
        self
    }

    /// Check if the Corner can be used in this case, based on the list of SlotStatus around the corner.
    pub fn is_usable_for(&self, slots: ClockWiseSlots) -> bool {
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
pub struct ClockWiseSlots {
    pub inner: [SlotStatus; 4],
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
