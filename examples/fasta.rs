use std::io::Read;

use miette::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

use transcriptase::{fasta::Fasta, genomics::genome::DnaSeq};

fn main() -> Result<()> {
    Registry::default()
        .with(
            tracing_tree::HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .init();
    let mut sra_data = String::new();
    let path = std::env::args().nth(1).expect("A Path must be provided");
    std::fs::File::open(path)
        .unwrap()
        .read_to_string(&mut sra_data)
        .unwrap();
    let seq = Fasta::<DnaSeq>::parse(&sra_data)?
        .pop()
        .expect("there is at least one sequence");
    println!("{:?}", seq.description);
    Ok(())
}
