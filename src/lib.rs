//! Ordinary Signal contract for the lojix deploy orchestrator component.
//!
//! `ethos/lib.ethos` is the canonical textual projection of one
//! authority-verified, role-free bootstrap Interface. Its checked Rust
//! projection carries only encoded identities. The operational ordinary
//! Input/Output role seating and Signal frame behavior remain handwritten Rust
//! until the language train reaches that behavior slice.

pub mod bootstrap_manifest;
pub mod schema;

/// Canonical textual projection of the ordinary Lojix Interface.
pub const LOJIX_INTERFACE_SOURCE: &str = include_str!("../ethos/lib.ethos");

/// Checked-in Rust projection of the same verified Interface transaction.
pub const LOJIX_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");
