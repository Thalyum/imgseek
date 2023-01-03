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

use colored::Colorize;
use flash_img_seeker::FlashImage;
use puzzle::{PuzzleDisplay, PuzzlePiece};
#[cfg(debug_assertions)]
use std::time::Instant;
use std::{convert::TryInto, fs};

fn main() -> anyhow::Result<()> {
    let matches = cli::build_cli().get_matches();

    // mandatory arguments
    let flash_img = matches.value_of("flash_image").unwrap();
    let bin_list = matches.values_of("binaries_list").unwrap();
    // argument with default value
    let bsize = matches.value_of("bsize").unwrap();
    let bsize: usize = bsize.parse::<usize>()?;
    // optional arguments
    let v_scale = matches.value_of("v_scale");
    let h_scale = matches.value_of("h_scale");

    #[cfg(debug_assertions)]
    let mut now = Instant::now();

    let flash_image = FlashImage::new(flash_img, bsize)?;

    #[cfg(debug_assertions)]
    {
        let elapsed = now.elapsed();
        println!("Flash image processed: {:.2?}", elapsed);
        now = Instant::now();
    }

    let mut puzzle = PuzzleDisplay::new(&flash_image, v_scale, h_scale);

    for binary_name in bin_list {
        let valid_offsets = flash_image.seek_image(&binary_name, bsize)?;
        if valid_offsets.is_empty() {
            let s = format!("➜ '{}' not found in '{}'...", binary_name, flash_img);
            println!("{}", s.bold());
            continue;
        } else {
            let s = format!("➜ '{}' found in '{}':", binary_name, flash_img);
            println!("{}", s.bold());

            let file_size = fs::metadata(&binary_name)?.len().try_into()?;
            for offset in valid_offsets.iter() {
                println!("\tfrom {:#010x} to {:#010x}", offset, offset + file_size);
                let p = PuzzlePiece::new(binary_name.to_owned(), file_size, *offset);
                puzzle.add_element(p)?;
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        let elapsed = now.elapsed();
        println!("All binaries processed: {:.2?}", elapsed);
        now = Instant::now();
    }

    if !puzzle.is_empty() {
        println!("{}", puzzle);
        #[cfg(debug_assertions)]
        {
            let elapsed = now.elapsed();
            println!("Schema displayed: {:.2?}", elapsed);
        }
    }

    Ok(())
}
