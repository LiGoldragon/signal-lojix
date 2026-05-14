# signal-lojix — architecture

*Typed Signal contract for the lojix deploy orchestrator.*

> **Status (2026-05-14):** first contract slice on
> `horizon-re-engineering`. The crate defines typed deploy,
> cache-retention, and generation-query records, declares the
> `signal-core` channel, and exposes Nix-backed round-trip and
> boundary tests.

## 0 · TL;DR

`signal-lojix` is the public vocabulary for cluster deploy
orchestration. It defines the typed request/reply records exchanged
between operator clients and the long-lived `lojix-daemon` binary in
the `lojix` repo over the daemon's Unix socket.

This repo owns records, validation newtypes, rkyv round trips, NOTA
round trips, and channel shape. It does not own the daemon
implementation, CLI behavior, storage tables, actors, socket lifecycle,
or the actual deploy pipeline.

> **Scope (eventual vs today).** This contract sits on today's stack:
> `signal-core` wire, rkyv archives, and `sema-engine` storage in
> consumers. The eventually-self-hosting stack is Sema-on-Sema; this
> contract is a realization step. See `~/primary/ESSENCE.md` §"Today
> and eventually".

## 1 · Channel Boundary

| Side | Component |
|---|---|
| Request producer | `lojix` CLI, future operator clients |
| Request consumer | `lojix-daemon` |
| Reply producer | `lojix-daemon` |
| Reply consumer | the caller that submitted the operation |

Transport: Unix socket at `/run/lojix/daemon.sock` carrying
`signal-core` frames. The transport itself belongs to `lojix`, not
this contract.

## 2 · Channel Surface

Per `~/primary/reports/system-assistant/04-dedicated-cloud-host-plan-second-revision.md`
§P5.4, the implemented channel has one `signal_channel!` declaration:

```rust
signal_channel! {
    request Request {
        DeploymentSubmission(DeploymentSubmission),
        CacheRetentionRequest(CacheRetentionRequest),
        GenerationQuery(GenerationQuery),
    }
    reply Reply {
        DeploymentAccepted(DeploymentAccepted),
        DeploymentRejected(DeploymentRejected),
        DeploymentObservation(DeploymentObservation),
        CacheRetentionAccepted(CacheRetentionAccepted),
        CacheRetentionRejected(CacheRetentionRejected),
        CacheRetentionObservation(CacheRetentionObservation),
        GenerationListing(GenerationListing),
    }
}
```

Daemon-internal actor messages, such as `LiveSet` to `GcRoot` events
or systemd-dbus container observations feeding the event log, stay
private to `lojix` and are not exported here. Boundary test: every
type in `signal-lojix` must be reachable from at least one socket
handler in a consumer.

## 3 · Record Families

### 3.1 Deployment

`DeploymentSubmission` carries the typed deploy request: cluster,
node, proposal source, flake reference, deployment plan, builder
selection, and substituters. The deployment plan is a sum-with-data
record:

- `FullOsDeployment { action }`
- `OsOnlyDeployment { action }`
- `HomeOnlyDeployment { user, mode }`

`BuilderSelection` is also a sum-with-data record:

- `BuildLocally`
- `DispatcherChoosesBuilder`
- `NamedBuilder { node }`

The daemon replies with `DeploymentAccepted`, `DeploymentRejected`,
or `DeploymentObservation`. Observations carry `DeploymentPhase`,
whose variants name the visible lifecycle stages: submitted,
building, built, closure-copying, activation-running,
activation-succeeded, and failed.

### 3.2 Cache Retention

`CacheRetentionRequest` carries a `GenerationId` plus a typed
`CacheRetentionAction`: pin, unpin, or retire. Replies are
`CacheRetentionAccepted`, `CacheRetentionRejected`, and
`CacheRetentionObservation`.

### 3.3 Generation Query

`GenerationQuery` asks for the live generation set with optional
cluster, node, and generation-kind filters. `GenerationListing`
carries `Generation` records: generation id, cluster, node, kind,
store path, and state.

## 4 · Boundary Rules

- Pure contract crate. No storage. No actors. No subprocesses. No
  socket lifecycle. No daemon code.
- Every record carries `NotaRecord`, `NotaEnum`, or `NotaSum` text
  projection plus rkyv binary-wire derives.
- Sum-with-data variants use variant-name == payload-type-name
  convention, matching the `signal-persona-*` precedent.
- Domain newtypes validate at construction.
- Naming follows `~/primary/skills/naming.md`.
- Request variants expose a contract-owned `sema_verb()` mapping:
  deployment submission is `Assert`, cache retention is `Mutate`, and
  generation query is `Match`.

## 5 · Tests

`nix flake check` is the canonical gate. The current checks include:

- `test-round-trip` — request/reply families round-trip through a
  length-prefixed `signal-core` frame.
- `test-sema-verb-mapping` — request variants carry the expected
  `signal-core::SignalVerb`.
- `test-contract-crate-has-no-runtime-dependencies` — the manifest
  does not depend on actor, storage, DBus, or async-runtime crates.
- `fmt`, `clippy`, `doc`, and doc tests.

## 6 · Code Map

```
Cargo.toml             # pure contract dependencies only
flake.nix              # Nix-backed package and checks
src/lib.rs             # records, validation newtypes, channel declaration
tests/round_trip.rs    # frame, NOTA, validation, and boundary tests
```

## 7 · Cross-Cutting Context

- Workspace `~/primary/ESSENCE.md` is upstream of every rule in this
  repo.
- The `signal-persona-*` family at `github:LiGoldragon/signal-persona-mind`,
  `signal-persona-message`, `signal-persona-router`, etc. is the
  structural precedent for shape.
- `lojix` at `github:LiGoldragon/lojix` is the consumer whose
  evolution drives this contract.
- Today's `lojix-cli` at `github:LiGoldragon/lojix-cli` is the current
  operator surface; it retires after CriomOS migrates to `lojix`.
