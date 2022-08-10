use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

use transcriptase::{err::TXaseResult, gff::GFF};

pub fn main() -> TXaseResult<()> {
    Registry::default()
        .with(
            tracing_tree::HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .init();
    let path = std::env::args()
        .skip(1)
        .next()
        .expect("A Path must be provided");
    let mut file = std::fs::File::open(path).unwrap();
    let _ = GFF::parse(&mut file)?;
    Ok(())
}
