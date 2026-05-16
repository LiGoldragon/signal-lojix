# signal-lojix — skills

The repo owns the typed Signal contract for the `lojix` deploy
orchestrator.

## Status

- First contract slice lives on the `horizon-re-engineering` branch.
- The canonical shape is in `ARCHITECTURE.md` §"Channel Surface".
- The typed startup configuration slice is active: daemon configuration
  is NOTA+rkyv via `impl_rkyv_configuration!`; CLI configuration is
  NOTA-only via `impl_nota_only_configuration!` and currently renders
  compact NOTA replies.

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

- `Request` / `Reply` / `Event` root enums emitted by
  `signal_core::signal_channel!`.
- Deployment records: `DeploymentSubmission`, `DeploymentAccepted`,
  `DeploymentRejected`, and `DeploymentObservation`.
- Cache-retention records: `CacheRetentionRequest`,
  `CacheRetentionAccepted`, `CacheRetentionRejected`, and
  `CacheRetentionObservation`.
- Generation query records: `GenerationQuery` and `GenerationListing`.
- Deployment and cache-retention observation subscription records,
  stream-open replies, stream-close replies, and stream events.
- Startup configuration records: `LojixDaemonConfiguration`,
  `LojixCliConfiguration`, `PeerDaemonBinding`, `SocketMode`, and
  `ReplyRendering`.
- Boundary newtypes such as `ClusterName`, `NodeName`, `DeploymentId`,
  `GenerationId`, `StorePath`, `DerivationPath`, `WirePath`,
  `UnixGroup`, and `OperatorIdentity`.

## Hard Rules

- Pure contract only: no Kameo actors, no daemon code, no DBus, no
  redb/sema tables, no subprocess execution, no socket accept loop.
- Every record crossing the channel derives rkyv plus NOTA projection
  (`NotaRecord`, `NotaEnum`, or `NotaSum`).
- Sum variants use the variant-name equals payload-type-name pattern.
- Request variants own their `SignalVerb` mapping through
  `Request::signal_verb()`.
- Observations are pushed stream events, not ordinary reply variants.
- Configuration records stay control-plane only. Requests carry deploy
  and query data; configuration carries sockets, paths, identity, and
  rendering policy.
- `nix flake check` is the review gate.
