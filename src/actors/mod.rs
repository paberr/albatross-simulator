use std::time::Duration;
use crate::datastructures::block::Block;

pub mod honest;

pub type MetricsEventType = ();

pub struct Timing {
    pub signature_verification: Duration,
}

impl Timing {
    pub fn block_processing_time(&self, block: &Block) -> Duration {
        Duration::default()
    }
}
