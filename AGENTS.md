You MUST read [~/primary/repos/lore/AGENTS.md](../../../lore/AGENTS.md) — the canonical agent contract.

# signal-lojix — agent carve-outs

- **Status: contract + typed configuration slice (2026-05-16).** The
  `horizon-re-engineering` branch has a Rust contract crate with deploy,
  cache-retention, generation-query, observation-stream, and
  `nota-config` startup configuration records plus Nix-backed tests.

- **Replacement infrastructure.** Per
  `~/primary/protocols/active-repositories.md` §"Replacement Stack
  (Future Infrastructure)", this repo replaces parts of the current
  `lojix-cli` once `lojix` ships. Do not assume current cluster deploys
  flow through it.

- **Pure contract.** Only typed records, validation newtypes, NOTA
  projection, rkyv archive shape, and contract tests belong here. No
  storage, no actors, no subprocesses, no socket lifecycle, no daemon
  code. Precedent: `signal-persona-mind`, `signal-persona-message`.

- **Spec is in `ARCHITECTURE.md` and in
  `~/primary/reports/system-assistant/04-dedicated-cloud-host-plan-second-revision.md`
  §P5.4.** Both must agree; ARCHITECTURE.md is the local source of
  truth.
