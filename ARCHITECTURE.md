# signal-lojix — architecture

*Typed Signal contract for the lojix deploy orchestrator.*

> **Status (2026-05-13):** Skeleton. Documentation only. No
> `Cargo.toml`, no `src/`, no `flake.nix`. Implementation lands when
> `lojix-daemon` development begins. The repo exists to lock the
> namespace and provide the canonical home for this architecture spec.
> See `protocols/active-repositories.md` §"Replacement Stack (Future
> Infrastructure)" in the primary workspace.

## 0 · TL;DR

`signal-lojix` is the public vocabulary for cluster deploy
orchestration. It defines the typed request/reply records exchanged
between operator clients (notably `lojix-cli`) and the long-lived
`lojix-daemon` over the daemon's Unix socket.

This repo owns records, validation newtypes, rkyv round trips, NOTA
round trips, and channel shape. It does not own the daemon
implementation (`lojix-daemon`), the CLI binary (`lojix-cli`), or the
actual deploy pipeline.

> **Scope (eventual vs today).** This contract sits on today's stack —
> `signal-core` wire, rkyv archives, `sema-db` storage in consumers.
> The eventually-self-hosting stack is Sema-on-Sema; this contract is
> a realization step. See `~/primary/ESSENCE.md` §"Today and
> eventually".

## 1 · Channel Boundary

| Side | Component |
|---|---|
| Request producer | `lojix-cli`, future operator clients |
| Request consumer | `lojix-daemon` |
| Reply producer | `lojix-daemon` |
| Reply consumer | the caller that submitted the operation |

Transport: Unix socket at `/run/lojix/daemon.sock` carrying
`signal-core` frames. The transport itself belongs to `lojix-daemon`,
not this contract.

## 2 · Planned Surface

```
// deployment surface
DeploymentSubmission      // client → daemon: "deploy this request"
DeploymentAccepted        // daemon → client: ack with deployment id
DeploymentRejected        // daemon → client: typed error
DeploymentObservation     // daemon → observers: phase events
                          //   Submitted, Building, Built, Copying,
                          //   Activating, Activated, Failed

// cache retention surface (operator-visible)
CacheRetentionRequest     // operator → daemon: pin/unpin/retire
CacheRetentionAccepted    // daemon → operator: ack with mutation id
CacheRetentionRejected    // daemon → operator: typed error
CacheRetentionObservation // daemon → observers: live-set changes

// queries
GenerationQuery           // any → daemon: "what's the live set?"
GenerationListing         // daemon → caller: live set
```

Daemon-internal actor messages (between `LiveSetActor` and
`GcRootActor`, container observers, etc.) stay private to
`lojix-daemon` and are **not** exported here. Boundary test: every
type in `signal-lojix` must be reachable from at least one socket
handler in a consumer.

## 3 · Boundary Rules

- Pure contract crate. No behavior. No storage. No actors. No I/O.
- Every record carries `NotaRecord` (text wire) + rkyv (binary wire)
  derives.
- Sum-with-data variants use variant-name == payload-type-name
  convention (precedent: `signal-persona-mind`, `signal-persona-message`).
- Domain newtypes validate at construction; foreign error types
  convert via `#[from]`.
- Naming follows `~/primary/skills/naming.md` (full English words; no
  crate-name prefix on types).

## 4 · Cross-Cutting Context

- Workspace `~/primary/ESSENCE.md` is upstream of every rule in this
  repo.
- The `signal-persona-*` family at `github:LiGoldragon/signal-persona-mind`,
  `signal-persona-message`, `signal-persona-router`, etc. is the
  structural precedent for shape.
- `lojix-daemon` at `github:LiGoldragon/lojix-daemon` is the consumer
  whose evolution drives this contract.
- Today's `lojix-cli` at `github:LiGoldragon/lojix-cli` is the current
  operator surface; it grows into a thin client over this contract
  once the daemon ships.
