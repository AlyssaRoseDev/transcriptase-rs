use std::io::Read;

use transcriptase::{
    fastq::{FastQ, Phred},
    genomics::genome::DnaSeq,
};

fn main() {
    let mut sra_data = String::new();
    let path = std::env::args()
        .skip(1)
        .next()
        .expect("A Path must be provided");
    std::fs::File::open(path)
        .unwrap()
        .read_to_string(&mut sra_data)
        .unwrap();
    let _: FastQ<DnaSeq, Phred> = FastQ::parse(&sra_data).unwrap();
}
