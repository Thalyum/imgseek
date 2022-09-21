//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("File is filled with 0: {0:?}")]
    ZeroFile(std::ffi::OsString),
    #[error("Cannot read file")]
    ReadErr,
}

pub type Result<T> = std::result::Result<T, Error>;
