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
> stack — `signal-core` wire kernel, rkyv archives, `sema-engine`
> typed database engine in the consumer daemon. The
> eventually-self-hosting stack is Sema-on-Sema; this contract is a
> realization step. See `~/primary/ESSENCE.md` §"Today and
> eventually".

## 1 · Channel Boundary

| Side | Component |
|---|---|
| Request producer | `lojix` CLI binary, future operator clients |
| Request consumer | `lojix-daemon` |
| Reply / event producer | `lojix-daemon` |
| Reply / event consumer | the caller that submitted the operation; subscribers |

Transport: Unix socket at `/run/lojix/daemon.sock` carrying
`signal-core` length-prefixed rkyv frames. The transport itself
belongs to the `lojix` repo, not this contract.

## 2 · Channel shape

One streaming channel — exchange operations plus a daemon-pushed
observation stream. Declared via `signal_channel!` per
`signal-core`'s `ARCHITECTURE.md` §3, with `request` / `reply` /
`event` / `stream` blocks. The grammar enforces verb tagging,
opens/belongs cross-references, and stream-relation witnesses at
compile time.

### Request variants and their SignalVerbs

| Variant | Verb | Purpose |
|---|---|---|
| `DeploymentSubmission` | `Assert` | Operator client → daemon: submit a deploy request. Daemon mints a `DeploymentIdentifier`. |
| `CacheRetentionRequest` | `Mutate` | Operator → daemon: pin / unpin / retire a generation. Mutates the live-set entry. |
| `GenerationQuery` | `Match` | Any client → daemon: read the live set (whole or filtered). |
| `DeploymentObservation` (subscribe) | `Subscribe` | Subscriber → daemon: stream phase events from one or all deploys. Opens `DeploymentEventStream`. |
| `CacheRetentionObservation` (subscribe) | `Subscribe` | Subscriber → daemon: stream cache-retention transitions. Opens `CacheRetentionEventStream`. |
| `StreamClose` | `Retract` | Subscriber → daemon: end an open subscription by token. Closes either stream. |

### Authority direction

Per `signal-core/ARCHITECTURE.md` §1 and
`~/primary/skills/contract-repo.md` §"Signal is the database
language", each verb encodes an authority direction:

- **`DeploymentSubmission` is `Assert`, not `Mutate`.** The
  operator submits a request that becomes a typed fact in the
  daemon's store; the daemon then evaluates the request against
  its criome-mediated authorization gate before any
  cluster-mutating effect runs. The verb is `Assert` because the
  submission is *"a new typed fact entered the system"*, not an
  authority order the daemon must obey-and-confirm. Authority for
  the actual deploy effect flows through criome (see
  `criome/ARCHITECTURE.md` §"Authorization model" and lojix's
  `CriomeAuthorizationActor`).
- **`CacheRetentionRequest` is `Mutate`** — an authority order
  from the operator to the daemon to change a generation's
  live-set entry (pin / unpin / retire). The daemon obeys and
  confirms; the operator transitions its own state from
  *possibly-mutated* to *now-mutated* on the typed reply.
- **`Subscribe` flows observer ↔ producer.** Observers up-tree
  subscribe to the daemon's pushed events (per `skills/push-not-pull.md`).
- **`StreamClose` is `Retract`** — subscriber-initiated; the
  subscriber retracts its own subscription. This is a self-retraction,
  not a top-down order.

### Reply variants

`DeploymentAccepted`, `DeploymentRejected`, `CacheRetentionAccepted`,
`CacheRetentionRejected`, `GenerationListing`, `StreamOpened`.

Replies do **not** declare their own `SignalVerb`. They are causally
tied to the request they answer; their legality is checked against
that request's operation. Per `~/primary/skills/contract-repo.md`
§"Reply discipline": if a future *"reply"* becomes a standalone
observation that travels independently (e.g., a long-lived
deployment-phase event observed by a subscriber other than the
issuing operator), it lands as its own request variant — `Assert`
for a new fact, `Subscribe` for a streaming observation — never as
a verb-less message.

### Event variants (streaming)

`DeploymentPhaseEvent` (belongs `DeploymentEventStream`): the typed
phases `Submitted`, `Building`, `Built`, `Copying`, `Activating`,
`Activated`, `Failed`. `CacheRetentionTransitionEvent` (belongs
`CacheRetentionEventStream`): live-set transitions as the daemon
rewrites GC roots.

### Streams

- `DeploymentEventStream` — opened by `DeploymentObservation` Subscribe;
  carries `DeploymentPhaseEvent` items; closed by `StreamClose`.
- `CacheRetentionEventStream` — opened by `CacheRetentionObservation`
  Subscribe; carries `CacheRetentionTransitionEvent` items; closed by
  `StreamClose`.

## 3 · Boundary Rules

- Pure contract crate. No behavior. No storage. No actors. No I/O.
- Channel shape is declared via one `signal_channel!` invocation so
  the macro emits the typed enums, verb witnesses, frame aliases
  (`StreamingFrame` / `StreamingFrameBody` since events ride this
  channel), stream-relation witnesses, and NOTA codecs.
- Every record carries `NotaRecord` (text wire) + rkyv (binary wire)
  derives. Verb wrapping and request-sequence brackets live at the
  `signal-core` kernel layer, not in this crate.
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
- Domain payload-to-verb mapping lives in this contract crate (in
  the `signal_channel!` declaration), not in `lojix` or its CLI.
  Per `~/primary/skills/contract-repo.md` §"Signal is the database
  language": *"Every cross-component Signal request declares its
  root verb. The verb is part of the contract."*

## 4 · Constraints

- Channel declares exactly the six SignalVerbs it uses; the macro
  rejects any verb keyword outside the six-root spine.
- Every `Subscribe` variant annotates `opens <StreamName>`.
- Every event variant annotates `belongs <StreamName>`.
- Every declared stream is opened by at least one `Subscribe`
  variant.
- Every event variant's `belongs` resolves to a stream whose
  `event` annotation points back at the same variant.
- A `StreamClose` variant tagged `Retract` carries the stream's
  token type as its payload.
- No `Unknown` variant on any closed enum. New domain shapes are
  coordinated schema bumps in this crate, not runtime escape
  hatches.
- Daemon-side typed errors decode through `DeploymentRejected` /
  `CacheRetentionRejected` payloads — no untyped error strings on
  the wire.

## 5 · Cross-Cutting Context

- Workspace `~/primary/ESSENCE.md` is upstream of every rule.
- `~/primary/skills/contract-repo.md` is the canonical discipline
  for contract crates — verb spine, named-relation discipline,
  reply discipline, layered-vs-base distinction, NOTA-on-contract-
  types. This contract follows it.
- `signal-core` at `github:LiGoldragon/signal-core` is the wire
  kernel. Two frame types: `ExchangeFrame` (no streams) and
  `StreamingFrame` (with streams); this channel uses
  `StreamingFrame` because it carries events.
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
