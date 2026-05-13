# signal-lojix

Typed Signal contract for the lojix deploy orchestrator. Defines the
records exchanged between operator clients and `lojix-daemon` over the
daemon's Unix socket.

**Status: skeleton.** Documentation only — no code yet. See
`ARCHITECTURE.md` for the planned shape and `~/primary/protocols/active-repositories.md`
for the replacement-stack context.

## Related

- `lojix-daemon` — the daemon that consumes this contract.
- `lojix-cli` — today's monolithic deploy orchestrator; becomes a thin
  client over `lojix-daemon` once the daemon ships.
- `signal-persona-mind`, `signal-persona-message` — structural
  precedents for the contract-crate shape.

## License

License of Non-Authority.
