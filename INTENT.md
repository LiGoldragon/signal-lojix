# INTENT — signal-lojix

*The wire vocabulary contract for the lojix deploy orchestrator.
Defines the typed request/reply/event channel that operator
clients (the thin `lojix` CLI) and future operator clients use to
submit deploys, manage generation retention, query the live set,
and subscribe to deploy/cache-retention observations. Companion to
`ARCHITECTURE.md`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-lojix`
contract. Workspace-shape intent stays in `primary/INTENT.md`.
Daemon-side intent stays in `lojix/INTENT.md`.

## Why this repo exists

`signal-lojix` is the **public peer-callable wire vocabulary** for
the `lojix-daemon`. It carries the typed request/reply/event
records for cluster deploy orchestration. The daemon
implementation, the CLI binary, the transport, and the deploy
pipeline live in the `lojix` crate; this crate owns records,
validation newtypes, the `signal_channel!`-declared channel shape,
and rkyv + NOTA round trips only.

**Lojix is not a persona component.** The mandatory `Tap`/`Untap`
observable block does not apply; deployment-observation and
cache-retention-observation are domain-specific `Watch`/`Unwatch`
pairs.

## The channel shape

One streaming channel — exchange operations plus a daemon-pushed
observation stream:

- **Requests:** `Deploy` (submit a deploy; daemon mints the
  `DeploymentIdentifier`), `Pin` / `Unpin` / `Retire` (generation
  retention — three distinct public actions, not collapsed under
  one cache-retention operation), `Query` (read the live set,
  whole or filtered), `WatchDeployments` / `WatchCacheRetention`
  (open observation streams), `Unwatch` (close a stream by token).
- **Replies:** verb-past-tense outcomes — `Deployed`, `Pinned` /
  `Unpinned` / `Retired`, `Queried(GenerationListing)`, `Watching`
  — plus typed `*Rejected` reasons.
- **Events:** `DeploymentPhaseEvent` (`Submitted`, `Building`,
  `Built`, `Copying`, `Activating`, `Activated`, `Failed`) and
  `CacheRetentionTransitionEvent`, each belonging to its stream.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs":

- Operation roots are domain verbs in verb form: `Deploy`, `Pin`,
  `Query`, `WatchDeployments` — never Sema class words (`Assert`,
  `Match`, `Mutate`) on the wire. The Sema class is something the
  daemon derives internally per operation, never something a peer
  names on the wire.
- Reply success variants are verb-past-tense matching the
  operation; rejections are `*Rejected` carrying a typed closed-enum
  reason.
- Payload records are the domain nouns the operation carries —
  full English words, no crate-name prefix. Per
  `primary/skills/naming.md`.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch; new domain
  shapes are coordinated schema bumps in this crate.
- The daemon mints `DeploymentIdentifier`, `StreamEventIdentifier`,
  and timestamps; request records do not carry them.
- Every subscription-shaped variant annotates `opens <Stream>`;
  every event variant annotates `belongs <Stream>`; the stream-close
  variant carries the stream's token type. The `signal_channel!`
  grammar enforces these cross-references at compile time.

## Constraints

- Pure contract crate: typed wire vocabulary, NOTA codecs, and
  round-trip witnesses only. No behavior, no storage, no actors, no
  I/O, no `tokio`.
- Every record carries `NotaRecord` (text wire) + rkyv (binary
  wire) derives; the same type IS the wire record AND the text
  record. Consumers carry no shadow types.
- Every operation and reply/event variant round-trips through both
  rkyv frames and NOTA text; witnesses live in the crate's tests.
- Wire dependency pins use named branches or tags, not raw revision
  hashes.

## Three-layer model

Layer 1 (this crate): contract operations on the wire (`Deploy`,
`Pin`, `Query`, `Watch*`). Layer 2 (daemon): component-local
`LojixCommand` records (e.g. `PlanBuild`, `CopyClosure`,
`ActivateGeneration`, `WriteLiveSetEntry`). Layer 3 (observation):
payloadless Sema class labels via `ToSemaOperation`, used only for
cross-component observation.

## Non-ownership

This crate does not own the `lojix` daemon runtime, actors, the GC
roots tree, the live-set or event-log tables, socket binding or
transport, the deploy pipeline, or the CLI binary. Those live in
`lojix`.

## See also

- `ARCHITECTURE.md` — detailed channel shape, stream relations,
  three-layer migration notes, closed-enum discipline.
- `../lojix/INTENT.md` — daemon-side intent.
- `primary/skills/contract-repo.md` — contract repo discipline.
- `primary/skills/component-triad.md` — repo triad structure and
  wire layers.
