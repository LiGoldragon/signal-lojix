# signal-lojix

The ordinary Signal contract for the Lojix deployment Nexus. The authored
contract is `ethos/signal.ethos`; Ethos Zero 28 generates the public `Input`
and `Output` roots, the named payload types, and `Answering`, one capability
per input, which a Nexus answers through `input.answer(&nexus)`. The build
rejects stale generated Rust.

`signal::Signalizable::signalize` archives an input or output into portable
rkyv bytes. A receiver constructs `signal::Signal<T>` from owned peer bytes
and restores the typed value with `signal::Restorable::restore`. Those kinds
and the length framing live in the `signal` repository, not here. The optional `datom` feature enables the authored text chain for
clients while the default contract has no Datom dependency.

Related repositories:

- `meta-signal-lojix` owns the owner-only Lojix Interface.
- `lojix` consumes both Interfaces and owns operational execution.
- `meta-signal-lojix` owns the privileged Lojix Signal contract.

## License

License of Non-Authority.
