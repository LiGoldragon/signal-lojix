You MUST read [~/primary/repos/lore/AGENTS.md](../../../lore/AGENTS.md) — the canonical agent contract.

# signal-lojix — agent carve-outs

- **Status: implemented contract (2026-06-07).** The live Rust crate
  is under `triad-port/`. Keep this repo pure: generated typed
  records, schema, codecs, and contract tests only. Daemon runtime,
  actors, process effects, storage, and CLI behavior live in `lojix`.

- **Replacement infrastructure.** Per
  `~/primary/protocols/active-repositories.md` §"Replacement Stack
  (Future Infrastructure)", this repo is the ordinary signal contract
  for the new `lojix` deploy stack. Do not put production deploy
  behavior here; that belongs to the `lojix` daemon.

- **Pure contract.** Only typed records and validation newtypes. No
  behavior, no storage, no actors, no I/O. Precedent:
  `signal-persona-mind`, `signal-persona-message`.

- **Spec.** `ARCHITECTURE.md` is the local source of truth.
