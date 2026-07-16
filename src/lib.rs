//! Ordinary Signal contract for the lojix deploy orchestrator component.
//!
//! Wire-only: the rkyv + NOTA codec and the signal-frame mail envelope for the
//! peer-callable read / observe / subscribe surface. Owner-only mutations
//! (Deploy/Pin/Unpin/Retire) live in `meta-signal-lojix`.
//!
//! This crate DEFINES the shared record types ONCE; `meta-signal-lojix`
//! cross-imports them via `signal-lojix:lib:TypeName`.

pub mod schema;

/// Canonical package roots consumed by package-qualified external schemas.
/// `schema::lib` remains the generated implementation module.
pub use schema::lib::{Input, Output};
