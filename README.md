# signal-lojix

The ordinary Lojix Interface: a strict Protos bootstrap document, its
authority-verified encoded Rust projection, and the operational behavior that
the current role-free bootstrap file kind cannot yet express.

The canonical source is `schema/lib.schema`. Build-time authority assembly
verifies that source and keeps `src/schema/lib/generated.rs` exactly fresh.
The generated Rust names are opaque encoded object coordinates; readable
contract heads remain visible through Dotos and the handwritten ordinary
request/reply role layer.

The crate publishes its explicit `schema/` directory through Cargo's
`ethos-source-dir` metadata protocol so downstream Interfaces consume the
producer-owned source directly. Runtime consumers do not depend on the
bootstrap compiler train.

Related repositories:

- `meta-signal-lojix` owns the owner-only Lojix Interface.
- `lojix` consumes both Interfaces and owns operational execution.
- `signal-frame` is the current binary frame substrate.

## License

License of Non-Authority.
