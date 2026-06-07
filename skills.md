# signal-lojix — skills

The live contract crate is under `triad-port/`.

## Status

- Pure contract code only: schema, generated typed records, codecs,
  and round-trip witnesses.
- No daemon runtime, process effects, storage, actors, or CLI behavior.
- The canonical specification is in `ARCHITECTURE.md`.

## Required reading when implementation starts

- `~/primary/skills/contract-repo.md` — the contract-crate pattern this
  repo follows.
- `~/primary/skills/typed-records-over-flags.md` — flag-soup-to-typed-
  records discipline.
- `~/primary/skills/naming.md` — full English words; no crate-name
  prefix on types.
- `~/primary/skills/rust/storage-and-wire.md` — NOTA + rkyv derive
  patterns.
- `signal-persona-mind`'s `skills.md` — closest structural precedent.
