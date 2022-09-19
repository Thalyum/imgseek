//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

use clap::{crate_authors, crate_description, crate_version, App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("flash-img-seeker")
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("flash_image")
                .short("i")
                .long("image")
                .required(true)
                .takes_value(true)
                .help("The flash image to search in"),
        )
        .arg(
            Arg::with_name("binaries_list")
                .short("b")
                .long("binaries")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .help("List of binaries to search for"),
        )
}
