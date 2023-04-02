//
// This file is part of imgseek
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Cannot read file")]
    ReadErr,
    #[error("Shape error: {0}")]
    Shape(#[from] ndarray::ShapeError),
    #[error("Free column not found")]
    FreeColNotFound,
    #[error("Bad shape")]
    BadShape,
}

pub type Result<T> = std::result::Result<T, Error>;
