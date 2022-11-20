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
    Identity,
    Used(usize),
}

impl SlotStatus {
    /// Returns `true` if the slot status is [`Free`].
    ///
    /// [`Free`]: SlotStatus::Free
    #[must_use]
    pub fn is_free(&self) -> bool {
        matches!(self, Self::Free)
    }

    /// Returns `true` if the slot status is [`Used`].
    ///
    /// [`Used`]: SlotStatus::Used
    #[must_use]
    pub fn is_used(&self) -> bool {
        matches!(self, Self::Used(_))
    }
}

//    +     | Free     | Identity | Used(b) |
// ---------+----------+----------+---------+
// Free     | Free     | Identity | Used(b) |
// Identity | Identity | Identity | Used(b) |
// Used(a)  | Used(a)  | Used(a)  | Panics! |
impl Add for SlotStatus {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.is_used() && self.is_used() {
            panic!("Cannot add Slotstatus::Used(a) with Slotstatus::Used(b)");
        }
        match self {
            SlotStatus::Free => rhs,
            _ => match rhs {
                SlotStatus::Used(_) => rhs,
                _ => self,
            },
        }
    }
}

//    *     | Free     | Identity | Used(b) |
// ---------+----------+----------+---------+
// Free     | Free     | Free     | Free    |
// Identity | Free     | Identity | Used(b) |
// Used(a)  | Free     | Used(a)  | Panics! |
impl Mul for SlotStatus {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            SlotStatus::Free => SlotStatus::Free,
            SlotStatus::Identity => rhs,
            SlotStatus::Used(_) => match rhs {
                SlotStatus::Free => SlotStatus::Free,
                SlotStatus::Identity => self,
                SlotStatus::Used(_) => {
                    panic!("Cannot mul Slotstatus::Used(a) with Slotstatus::Used(b)")
                }
            },
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
        Self::Identity
    }

    fn is_one(&self) -> bool {
        self.is_used()
    }
}

// Implementation of 'Display' trait
impl fmt::Display for SlotStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlotStatus::Free => write!(f, "░"),
            SlotStatus::Used(a) => write!(f, "{}", a),
            SlotStatus::Identity => write!(f, "▓"),
        }
    }
}
