//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

mod cli;
mod error;
mod flash_img_seeker;
mod puzzle;

use flash_img_seeker::{process_flash_img, seek_image};
use puzzle::{PuzzleDisplay, PuzzlePiece};
use std::{convert::TryInto, fs};

fn main() -> anyhow::Result<()> {
    let matches = cli::build_cli().get_matches();

    // mandatory arguments
    let flash_img = matches.value_of("flash_image").unwrap();
    let bin_list = matches.values_of("binaries_list").unwrap();
    // argument with default value
    let bsize = matches.value_of("bsize").unwrap();
    let bsize: usize = bsize.parse::<usize>()?;

    let flash_hash_table = process_flash_img(flash_img, bsize)?;
    let flash_img_size = fs::metadata(flash_img)?.len();

    let mut puzzle = PuzzleDisplay::new(flash_img_size);

    for binary_name in bin_list {
        let valid_offsets = seek_image(&flash_hash_table, &binary_name, bsize)?;
        let file_size = fs::metadata(&binary_name)?.len().try_into()?;

        for offset in valid_offsets.iter() {
            let p = PuzzlePiece::new(binary_name.to_owned(), file_size, *offset);

            puzzle.add_element(p);
        }
    }

    // puzzle.compute_all_indexes();

    println!("{:x?}", puzzle);
    println!("{}", puzzle);

    Ok(())
}
