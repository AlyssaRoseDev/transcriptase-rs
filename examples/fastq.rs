use std::io::Read;

use miette::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

use transcriptase::{
    fastq::{FastQ, Phred},
    genomics::genome::DnaSeq,
};

fn main() -> Result<()> {
    Registry::default()
        .with(
            tracing_tree::HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .init();
    let mut sra_data = String::new();
    let path = std::env::args()
        .skip(1)
        .next()
        .expect("A Path must be provided");
    std::fs::File::open(path)
        .unwrap()
        .read_to_string(&mut sra_data)
        .unwrap();
    let _: FastQ<DnaSeq, Phred> = FastQ::parse(&sra_data)?;
    Ok(())
}
