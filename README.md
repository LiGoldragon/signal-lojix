# signal-lojix

Typed Signal contract for the lojix deploy orchestrator. Defines the
records exchanged between operator clients and `lojix-daemon` over the
daemon's Unix socket.

**Status:** first contract slice implemented on the
`horizon-re-engineering` branch. The crate exposes typed deploy,
cache-retention, and generation-query records, plus Nix-backed
round-trip and boundary tests.

## Related

- `lojix` — the daemon/CLI crate that consumes this contract.
- `lojix-cli` — today's monolithic deploy orchestrator; retires after
  CriomOS migrates to `lojix`.
- `signal-persona-mind`, `signal-persona-message` — structural
  precedents for the contract-crate shape.

## License

License of Non-Authority.
