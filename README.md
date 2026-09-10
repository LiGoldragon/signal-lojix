# signal-lojix

The ordinary Signal contract for the Lojix deployment Nexus. The authored
contract is `ethos/signal.ethos`; Ethos Zero generates the public `Query`,
`Response`, and named payload types and the build rejects stale generated Rust.

`Signalizable::signalize` archives a query or response into portable rkyv
bytes. A receiver constructs `Signal<T>` from owned peer bytes and restores
the typed value with `Restorable::restore`. Transport length framing belongs
to the Nexus. The optional `datom` feature enables the authored text chain for
clients while the default contract has no Datom dependency.

Related repositories:

- `meta-signal-lojix` owns the owner-only Lojix Interface.
- `lojix` consumes both Interfaces and owns operational execution.
- `meta-signal-lojix` owns the privileged Lojix Signal contract.

## License

License of Non-Authority.
