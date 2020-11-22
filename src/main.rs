#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod error;
mod genomics;
mod cli;
mod formats;

use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::Path,
};

use structopt::StructOpt;

use crate::{
    error::{TxaseErr, TxaseResult},
    genomics::prelude::*,
    cli::*
};

fn main() {
    let c = CliOpts::from_args();
    let testgen =
        Genome::open_and_parse(c.get_path(), None)
            .unwrap();
    println!("{}", testgen)
}
