# signal-lojix architecture

`signal-lojix` owns the ordinary Lojix Interface and its current Rust runtime
projection. The contract is a Protos source boundary first; Rust and the
present frame substrate are consumers of that boundary, not assumptions in
the Interface.

## Authority boundary

`schema/lib.schema` is the canonical `Interface.{1 0 0}` document. It is a
strict, role-free bootstrap document: imports, Nexus, Sema, and roles are
empty, and the Types section contains the whole ordinary vocabulary.

`src/bootstrap_manifest.rs` records the producer-owned authority identity,
opaque declaration seats, and canonical order. A build assembles that state,
verifies the source against it, and asks `schema-rust` for the Rust Logos
projection. The resulting `src/schema/lib/generated.rs` contains encoded
object coordinates rather than source spellings. It is checked for exact
freshness on every ordinary build and can be updated only with
`SIGNAL_LOJIX_UPDATE_INTERFACE_ARTIFACTS=1`.

The build publishes the explicit `schema/` directory through Cargo's
`ethos-source-dir` metadata protocol. A downstream Interface therefore reads
the producer's canonical source directory; it does not receive a copied
schema description or a second naming policy.

## Current-stage behavior

The bootstrap file kind does not yet express operational roles or wire
behavior. `src/schema/lib/behavior.rs` supplies only that missing behavior by
hand:

- structural runtime traits over the generated encoded types;
- readable Dotos spellings for human, agent, harness, and GUI surfaces;
- the ordinary request and reply role seating;
- the allocated `signal-frame` contract boundary.

These definitions do not mint structural types and do not alter authority.
The Interface remains role-free until role-bearing bootstrap documents are
available. No behavior is silently inferred from an empty role section.

## Runtime boundary

Normal dependencies contain only the runtime projection: `rkyv`,
`signal-frame`, and `thiserror`, with Dotos available through the explicit
`dotos-text` feature. Authority assembly, translation, and Rust projection
crates are build dependencies only. This repository owns no process,
transport selection, storage, deployment execution, or operating-system
policy.

The ordinary roots are the encoded projections of `OrdinaryRequest` and
`OrdinaryReply`. Their handwritten role mappings preserve contract-local
heads such as `Query`, `WatchDeployments`, `Unwatch`, `Queried`, and
`Unwatched`; those readable heads are Dotos surface vocabulary, not Rust
identity aliases.

## Change law

Structural change begins in the Interface and its authority manifest, then
regenerates the Rust projection. Operational behavior changes alongside the
handwritten role and wire layer. Wire-breaking changes allocate a new wire
revision and a new crate version. No compatibility aliases, alternate schema
readers, or permissive fallbacks are carried forward.
