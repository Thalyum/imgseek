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
use std::{
    convert::TryInto,
    fs, panic,
    sync::{Arc, Mutex},
    thread,
};

fn main() -> anyhow::Result<()> {
    let matches = cli::build_cli().get_matches();

    // mandatory arguments
    let flash_img = matches.value_of("flash_image").unwrap();
    let bin_list = matches.values_of("binaries_list").unwrap();
    // argument with default value
    let bsize: usize = matches.value_of("bsize").unwrap().parse::<usize>()?;
    // optional arguments
    let v_scale = matches.value_of("v_scale");
    let h_scale = matches.value_of("h_scale");

    #[cfg(debug_assertions)]
    let mut now = Instant::now();

    // RO thread-shared structure
    let flash_image = Arc::new(FlashImage::new(flash_img, bsize)?);
    #[cfg(debug_assertions)]
    {
        let elapsed = now.elapsed();
        println!("Flash image processed: {:.2?}", elapsed);
        now = Instant::now();
    }

    // create a mutable thread-shared PuzzleDisplay
    let puzzle = Arc::new(Mutex::new(PuzzleDisplay::new(
        &flash_image,
        v_scale,
        h_scale,
    )));

    // thread 'pool'
    let mut threads: Vec<_> = Vec::new();

    for binary_name in bin_list {
        let binary_name = binary_name.to_string();
        // clone shared references
        let flash_image: Arc<FlashImage> = Arc::clone(&flash_image);
        let puzzle = Arc::clone(&puzzle);
        // here is the thread
        let handle = thread::spawn(move || -> thread::Result<()> {
            println!("DBG: thread spawned for {}", binary_name);
            let valid_offsets = flash_image.seek_image(&*binary_name, bsize).unwrap();
            if valid_offsets.is_empty() {
                let s = format!("➜ '{}' not found in flash image...", binary_name);
                println!("{}", s.bold());
            } else {
                // FIXME: format one string beforehand, then print it in one go
                // otherwise the print of various threads may be mixed
                let s = format!("➜ '{}' found in flash image:", binary_name);
                println!("{}", s.bold());

                // FIXME: replaced '?' by 'unwrap()' because was unable to transform
                // custom Error into thread::Error
                // TODO: use anyhow::Error everywhere instead of thiserror
                let file_size = fs::metadata(&*binary_name)
                    .unwrap()
                    .len()
                    .try_into()
                    .unwrap();
                for offset in valid_offsets.iter() {
                    println!("\tfrom {:#010x} to {:#010x}", offset, offset + file_size);
                    let p = PuzzlePiece::new(binary_name.to_string(), file_size, *offset);
                    puzzle.lock().unwrap().add_element(p).unwrap();
                }
            };
            println!("DGB: thread finished for {}", binary_name);
            Ok(())
        });
        threads.push(handle);
    }

    // Join every thread in the pool: find every binaries
    for handle in threads.into_iter() {
        if let Err(e) = handle.join().unwrap() {
            panic::resume_unwind(e);
        }
    }
    #[cfg(debug_assertions)]
    {
        let elapsed = now.elapsed();
        println!("All binaries processed: {:.2?}", elapsed);
        now = Instant::now();
    }

    // display the flash layout
    if !puzzle.lock().unwrap().is_empty() {
        println!("{}", puzzle.lock().unwrap());
        #[cfg(debug_assertions)]
        {
            let elapsed = now.elapsed();
            println!("Schema displayed: {:.2?}", elapsed);
        }
    }

    Ok(())
}
