# signal-lojix

Typed Signal contract for the lojix deploy orchestrator. Defines the
ordinary peer-callable records exchanged between operator clients and
`lojix-daemon` over the daemon's ordinary Unix socket.

**Status: implemented contract.** The repo contains the schema,
generated Rust records/codecs, and contract witnesses. Runtime deploy
behavior lives in `lojix`.

## Related

- `lojix-daemon` — the daemon that consumes this ordinary contract.
- `meta-signal-lojix` — owner-only policy contract for deploy and
  retention mutations.
- `signal-frame` — frame kernel used by this contract.
- `signal-persona-mind`, `signal-persona-message` — structural
  precedents for the contract-crate shape.

## License

License of Non-Authority.
