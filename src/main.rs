//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

mod cli;
mod flash_img_seeker;

use flash_img_seeker::*;

fn main() -> anyhow::Result<()> {
    let matches = cli::build_cli().get_matches();

    // mandatory arguments
    let flash_img = matches.value_of("flash_image").unwrap();
    let bin_list = matches.values_of("binaries_list").unwrap();
    // argument with default value
    let bsize = matches.value_of("bsize").unwrap();
    let bsize: usize = bsize.parse::<usize>()?;

    let flash_hash_table = process_flash_img(flash_img, bsize)?;

    for binaries in bin_list {
        seek_image(&flash_hash_table, &binaries, bsize)?;
    }

    Ok(())
}
