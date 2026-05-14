You MUST read [~/primary/repos/lore/AGENTS.md](../../../lore/AGENTS.md) — the canonical agent contract.

# signal-lojix — agent carve-outs

- **Status: skeleton (2026-05-13).** No `Cargo.toml`, no `src/`, no
  `flake.nix`, no `skills.md` body beyond a stub. The repo exists to
  lock the namespace and host the architecture spec. Do not begin
  implementation here without explicit direction from the user; the
  implementation kickoff is gated on `lojix-daemon` work starting.

- **Future infrastructure.** Per
  `~/primary/protocols/active-repositories.md` §"Replacement Stack
  (Future Infrastructure)", this repo replaces parts of the current
  `lojix-cli` once `lojix-daemon` ships. Do not assume current cluster
  deploys flow through it.

- **Pure contract.** When implementation lands, only typed records and
  validation newtypes. No behavior, no storage, no actors, no I/O.
  Precedent: `signal-persona-mind`, `signal-persona-message`.

- **Spec.** `ARCHITECTURE.md` is the local source of truth.
