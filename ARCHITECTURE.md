# signal-lojix architecture

`ethos/signal.ethos` is the sole structural authority. Ethos Zero generates
all contract types, including the `Query` and `Response` roots. Build-time
freshness prevents handwritten generated Rust.

The default crate contains only rkyv contract support. `Signal<T>` carries raw
portable archive bytes and no legacy envelope. The runtime owns stream length
framing and socket behavior. The `datom` feature adds final Protos/Datom
derives for client text input; it is deliberately absent from the default
daemon dependency graph.

Contract behavior is trait-borne, and the traits live in `signal`:
`Signalizable`, `ByteViewable`, and `Restorable`. Received bytes enter
through `From<Vec<u8>> for Signal<T>`. This crate implements none of them —
they are blanket implementations over every rkyv contract type.
Schema changes start in the Ethos source and regenerate the projection; no
compatibility aliases or alternate readers are retained.
