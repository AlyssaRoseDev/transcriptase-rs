use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

use miette::Result;
use transcriptase::gff::GFF;

pub fn main() -> Result<()> {
    Registry::default()
        .with(
            tracing_tree::HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .init();
    let path = std::env::args().nth(1).expect("A Path must be provided");
    let mut file = std::fs::File::open(path).unwrap();
    let gff = GFF::read_from(&mut file)?;
    info!(
        "Parsed {} sequence(s) with {} entries",
        gff.metadata.sequence_regions.unwrap().len(),
        gff.entries.len()
    );
    Ok(())
}
