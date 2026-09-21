pub mod generated;
pub mod horizon_wire;
pub mod horizon_wire_types;
pub use generated::signal::*;
pub use horizon_wire_types::ClusterProposalWire;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
