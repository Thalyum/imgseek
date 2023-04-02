//
// This file is part of imgseek
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

#[macro_use]
extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = std::env::var_os("OUT_DIR").unwrap();
    let mut app = build_cli();
    for shell in &[Shell::Bash, Shell::Zsh] {
        app.gen_completions(crate_name!(), *shell, &outdir);
    }
}
