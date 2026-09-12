# signal-lojix

The ordinary Signal contract for the Lojix deployment Nexus. The authored
contract is `ethos/signal.ethos`; Ethos Zero generates the public `Query`,
`Response`, and named payload types and the build rejects stale generated Rust.

`signal::Signalizable::signalize` archives a query or response into portable
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
