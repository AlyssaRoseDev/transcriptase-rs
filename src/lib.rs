#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod error;
mod genomics;
mod formats;

use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    error::{TxaseErr, TxaseResult},
    genomics::prelude::*,
};

mod tests {

    #[test]
    fn load_test() {
        
    }

}