//
// This file is part of imgseek
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

use clap::{crate_authors, crate_description, crate_version, App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("imgseek")
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
        .arg(
            Arg::with_name("bsize")
                .short("s")
                .long("size")
                .takes_value(true)
                .default_value("512")
                .help("Page / block size"),
        )
        .arg(
            Arg::with_name("v_scale")
                .long("v_scale")
                .takes_value(true)
                .help("Vertical scaling, default is half of the term size"),
        )
        .arg(
            Arg::with_name("h_scale")
                .long("h_scale")
                .takes_value(true)
                .help("Horizontal scaling, default is half of the term size"),
        )
}
