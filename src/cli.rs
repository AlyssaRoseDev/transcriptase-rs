// use std::path::PathBuf;

// use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// #[structopt(name = "")]
// pub struct CliOpts {
//     #[structopt(short("i"), long, parse(from_os_str))]
//     infile: PathBuf,
//     #[structopt(short, long)]
//     verbose: Option<bool>
// }

// impl CliOpts {
//     pub fn get_path(&self) -> &PathBuf {
//         return &self.infile
//     } 
// }