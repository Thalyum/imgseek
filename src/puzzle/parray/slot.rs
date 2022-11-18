//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//
use num_traits::{identities::Zero, One};
use std::{
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign},
};

#[derive(Debug, Clone, Copy)]
pub enum SlotStatus {
    Free,
    Used,
}

impl SlotStatus {
    /// Returns `true` if the slot status is [`Free`].
    ///
    /// [`Free`]: SlotStatus::Free
    #[must_use]
    fn is_free(&self) -> bool {
        matches!(self, Self::Free)
    }

    /// Returns `true` if the slot status is [`Used`].
    ///
    /// [`Used`]: SlotStatus::Used
    #[must_use]
    fn is_used(&self) -> bool {
        matches!(self, Self::Used)
    }
}

// '+' implementation is a logical 'or'
// Free + Free = Free
// Free + Used = Used
// Used + Free = Used
// Used + Used = Used
impl Add for SlotStatus {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.is_free() && self.is_free() {
            SlotStatus::Free
        } else {
            SlotStatus::Used
        }
    }
}

// '+' implementation is a logical 'and'
// Free * Free = Free
// Free * Used = Free
// Used * Free = Free
// Used * Used = Used
impl Mul for SlotStatus {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if rhs.is_used() && self.is_used() {
            SlotStatus::Used
        } else {
            SlotStatus::Free
        }
    }
}

// Implementation of '+='
impl AddAssign for SlotStatus {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// Implementation of '*='
impl MulAssign for SlotStatus {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// Implementation of 'Zero' trait
impl Zero for SlotStatus {
    fn zero() -> Self {
        Self::Free
    }

    fn is_zero(&self) -> bool {
        self.is_free()
    }
}

// Implementation of 'One' trait
impl One for SlotStatus {
    fn one() -> Self {
        Self::Used
    }

    fn is_one(&self) -> bool {
        self.is_used()
    }
}

// Implementation of 'Display' trait
impl fmt::Display for SlotStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            SlotStatus::Free => "░",
            SlotStatus::Used => "▓",
        };
        write!(f, "{}", symbol)
    }
}
