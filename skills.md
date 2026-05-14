# signal-lojix — skills

The repo owns the typed Signal contract for the `lojix` deploy
orchestrator.

## Status

- First contract slice lives on the `horizon-re-engineering` branch.
- The canonical shape is in `ARCHITECTURE.md` §"Channel Surface" and
  `~/primary/reports/system-assistant/04-dedicated-cloud-host-plan-second-revision.md`
  §P5.4.

## Required Reading

- `~/primary/skills/contract-repo.md` — the contract-crate pattern this
  repo follows.
- `~/primary/skills/typed-records-over-flags.md` — flag-soup-to-typed-
  records discipline.
- `~/primary/skills/naming.md` — full English words; no crate-name
  prefix on types.
- `~/primary/skills/rust/storage-and-wire.md` — NOTA + rkyv derive
  patterns.
- `signal-persona-mind`'s `skills.md` — closest structural precedent.

## Owned Surface

- `Request` / `Reply` root enums emitted by `signal_core::signal_channel!`.
- Deployment records: `DeploymentSubmission`, `DeploymentAccepted`,
  `DeploymentRejected`, and `DeploymentObservation`.
- Cache-retention records: `CacheRetentionRequest`,
  `CacheRetentionAccepted`, `CacheRetentionRejected`, and
  `CacheRetentionObservation`.
- Generation query records: `GenerationQuery` and `GenerationListing`.
- Boundary newtypes such as `ClusterName`, `NodeName`, `DeploymentId`,
  `GenerationId`, `StorePath`, and `DerivationPath`.

## Hard Rules

- Pure contract only: no Kameo actors, no daemon code, no DBus, no
  redb/sema tables, no subprocess execution, no socket accept loop.
- Every record crossing the channel derives rkyv plus NOTA projection
  (`NotaRecord`, `NotaEnum`, or `NotaSum`).
- Sum variants use the variant-name equals payload-type-name pattern.
- Request variants own their `SignalVerb` mapping through
  `Request::sema_verb()`.
- `nix flake check` is the review gate.
