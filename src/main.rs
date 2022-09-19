//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

mod cli;
mod flash_img_seeker;

fn main() -> anyhow::Result<()> {
    let matches = cli::build_cli().get_matches();

    // Insert code here

    Ok(())
}
