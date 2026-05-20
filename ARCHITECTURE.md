# signal-lojix — architecture

*Typed Signal contract for the lojix deploy orchestrator.*

> **Status (2026-05-15):** Skeleton. Documentation only. No
> `Cargo.toml`, no `src/`, no `flake.nix`. Implementation lands when
> `lojix` daemon development begins on the `horizon-re-engineering`
> feature branch. The repo exists to lock the namespace and provide
> the canonical home for this architecture spec. See
> `~/primary/protocols/active-repositories.md` §"Replacement Stack".

## 0 · TL;DR

`signal-lojix` is the public wire vocabulary for cluster deploy
orchestration. It defines the typed request/reply/event records
exchanged between operator clients (the thin `lojix` CLI binary) and
the long-lived `lojix-daemon` over a Unix socket.

This crate owns records, validation newtypes, the
`signal_channel!`-declared channel shape, rkyv round trips, and NOTA
round trips. It does not own the daemon implementation, the CLI
binary, or the deploy pipeline — those live in the `lojix` crate.

> **Scope (today vs eventually).** This contract sits on today's
> stack — `signal-frame` wire kernel, rkyv archives, `sema-engine`
> typed database engine in the consumer daemon. The
> eventually-self-hosting stack is Sema-on-Sema; this contract is a
> realization step. See `~/primary/ESSENCE.md` §"Today and
> eventually".

## MUST IMPLEMENT — three-layer migration

This contract is migrating to the three-layer model affirmed
2026-05-20 per
`primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
and `primary/reports/designer/248-three-layer-changes-for-operators.md`.

This crate is currently skeleton (no `Cargo.toml`, no `src/`). When
implementation begins, drop the SignalVerb prefixes entirely.

**Layer 1 — Contract Operations on the wire (this crate).** Candidate
contract-local verbs per the existing variant table: `Deploy`
(verb-form; payload becomes the deploy request noun, not
`DeploymentSubmission`), `Pin` / `Unpin` / `Retire` (verb-form
splits of `CacheRetentionRequest` — three distinct public actions
should not collapse under one cache-retention operation), `Query`
(for `GenerationQuery`, payload names the filter shape), `Watch`
(for the two subscribe variants; payload distinguishes deployment
events vs cache-retention events), `Unwatch` (for `StreamClose`).

**Lojix is not a persona component.** The mandatory `Tap`/`Untap`
observable block does not apply. The existing
deployment-observation and cache-retention-observation subscriptions
stay as domain-specific Watch/Unwatch pairs.

**Layer 2 — Component Commands (lojix-daemon crate).** Lojix's
daemon owns its typed Command enum (e.g. `LojixCommand::PlanBuild`,
`LojixCommand::CopyClosure`, `LojixCommand::ActivateGeneration`,
`LojixCommand::WriteLiveSetEntry`) plus a `CommandExecutor` that
knows the deploy pipeline and the live-set tables.

**Layer 3 — Sema classification (signal-sema).** Each Component
Command projects to a payloadless `SemaOperation` class label via
`ToSemaOperation`. Lojix does not import payload-bearing Sema
variants.

**Frame layer.** The dependency on `signal-core` shifts to
`signal-frame` once the new crate is published.

References:
- `primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
- `primary/reports/designer/248-three-layer-changes-for-operators.md`
- `primary/skills/component-triad.md` §"Verbs come in three layers"
- `primary/skills/contract-repo.md` §"Public contracts use contract-local operation verbs"

**Note to remover:** when the implementation lands with the new
shape, remove this section and add a `## Migration history —
three-layer model (2026-05-XX)` paragraph noting the shape.

## 1 · Channel Boundary

| Side | Component |
|---|---|
| Request producer | `lojix` CLI binary, future operator clients |
| Request consumer | `lojix-daemon` |
| Reply / event producer | `lojix-daemon` |
| Reply / event consumer | the caller that submitted the operation; subscribers |

Transport: Unix socket at `/run/lojix/daemon.sock` carrying
`signal-frame` length-prefixed rkyv frames. The transport itself
belongs to the `lojix` repo, not this contract.

## 2 · Channel shape

One streaming channel — exchange operations plus a daemon-pushed
observation stream. Declared via `signal_channel!` per
`signal-frame`'s `ARCHITECTURE.md`, with `request` / `reply` /
`event` / `stream` blocks. The grammar enforces opens/belongs
cross-references and stream-relation witnesses at compile time.

### Contract operations (Layer 1) and their Sema-class projections (Layer 3)

The wire form carries the contract-local verb only; the Sema class
label below is the *expected daemon-side classification* used for
cross-component observation.

| Operation | Purpose | Expected Sema class |
|---|---|---|
| `Deploy` | Operator client → daemon: submit a deploy request. Daemon mints a `DeploymentIdentifier`. | `Assert` |
| `Pin` / `Unpin` / `Retire` | Operator → daemon: pin / unpin / retire a generation. Mutates the live-set entry. | `Mutate` (Pin/Unpin), `Retract` (Retire) |
| `Query` | Any client → daemon: read the live set (whole or filtered). | `Match` |
| `WatchDeployments` | Subscriber → daemon: stream phase events from one or all deploys. Opens `DeploymentEventStream`. | `Subscribe` |
| `WatchCacheRetention` | Subscriber → daemon: stream cache-retention transitions. Opens `CacheRetentionEventStream`. | `Subscribe` |
| `Unwatch` | Subscriber → daemon: end an open subscription by token. Closes either stream. | `Retract` |

### Reply variants

Verb-past-tense for outcomes plus typed rejection payloads (per
reply discipline in `skills/contract-repo.md`): `Deployed`,
`DeployRejected`, `Pinned` / `Unpinned` / `Retired`,
`Queried(GenerationListing)`, `Watching` (subscription opened), and
the typed `*Rejected` reasons.

### Event variants (streaming)

`DeploymentPhaseEvent` (belongs `DeploymentEventStream`): the typed
phases `Submitted`, `Building`, `Built`, `Copying`, `Activating`,
`Activated`, `Failed`. `CacheRetentionTransitionEvent` (belongs
`CacheRetentionEventStream`): live-set transitions as the daemon
rewrites GC roots.

### Streams

- `DeploymentEventStream` — opened by `WatchDeployments`; carries
  `DeploymentPhaseEvent` items; closed by `Unwatch`.
- `CacheRetentionEventStream` — opened by `WatchCacheRetention`;
  carries `CacheRetentionTransitionEvent` items; closed by `Unwatch`.

## 3 · Boundary Rules

- Pure contract crate. No behavior. No storage. No actors. No I/O.
- Channel shape is declared via one `signal_channel!` invocation so
  the macro emits the typed enums, frame aliases (`StreamingFrame` /
  `StreamingFrameBody` since events ride this channel),
  stream-relation witnesses, and NOTA codecs.
- Every record carries `NotaRecord` (text wire) + rkyv (binary wire)
  derives. Request-sequence brackets live at the `signal-frame`
  kernel layer, not in this crate.
- Sum-with-data variants use the variant-name == payload-type-name
  convention (precedent: `signal-persona-mind`,
  `signal-persona-message`).
- Domain newtypes validate at construction; foreign error types
  convert via `#[from]`.
- Naming follows `~/primary/skills/naming.md` (full English words;
  no crate-name prefix on types).
- Daemon-internal actor messages stay private to the `lojix` crate
  and are not exported here. Boundary test: every type in
  `signal-lojix` is reachable from at least one socket handler in
  the consumer daemon.

## 4 · Constraints

- Every contract-local operation is a verb in verb form; the macro
  emits the NOTA codec keyed on the payload's record head.
- Every subscription-shaped variant annotates `opens <StreamName>`.
- Every event variant annotates `belongs <StreamName>`.
- Every declared stream is opened by at least one subscription-shaped
  variant.
- Every event variant's `belongs` resolves to a stream whose
  `event` annotation points back at the same variant.
- A stream-close variant (e.g. `Unwatch`) carries the stream's
  token type as its payload; the macro grammar enforces a request-side
  `Retract`-shaped variant for close.
- No `Unknown` variant on any closed enum. New domain shapes are
  coordinated schema bumps in this crate, not runtime escape
  hatches.
- Daemon-side typed errors decode through `DeployRejected` /
  `PinRejected` (etc.) payloads — no untyped error strings on the
  wire.
- Sema classification projections live in the lojix daemon
  (Component Commands impl `ToSemaOperation`), not in this contract
  crate.

## 5 · Cross-Cutting Context

- Workspace `~/primary/ESSENCE.md` is upstream of every rule.
- `signal-frame` at `github:LiGoldragon/signal-frame` is the wire
  kernel. Two frame types: `ExchangeFrame` (no streams) and
  `StreamingFrame` (with streams); this channel uses
  `StreamingFrame` because it carries events.
- `signal-sema` at `github:LiGoldragon/signal-sema` owns the
  payloadless classification labels used at the observation layer.
- `lojix` at `github:LiGoldragon/lojix` is the daemon implementation
  whose evolution drives this contract. Both binaries (the
  long-lived `lojix-daemon` orchestrator and the thin `lojix` CLI
  client) live in that crate.
- The `signal-persona-*` family at
  `github:LiGoldragon/signal-persona-mind`,
  `signal-persona-message`, `signal-persona-router`, etc., is the
  structural precedent for shape.
- Today's `lojix-cli` at `github:LiGoldragon/lojix-cli` is the
  legacy monolithic orchestrator. It stays at its current schema
  until CriomOS migrates to consume this daemon's projection, then
  retires — it does not gradually grow into a client of this
  contract.
