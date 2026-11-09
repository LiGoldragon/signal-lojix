# Upgrades

# 0.10.0 to 0.10.1

`horizon-lib` moves from 0.17.0 (`187b028a1c1ec6c8926a618a3ca435440d077ae7`)
to 0.17.1 (`e723ce117104557a6eb88555008a68627adae313`), a revision with no
type changes: the `isGitServer` gate no longer requires the node's base public
keys. No signal-lojix type changes, and `src/generated/signal.rs` is
byte-identical.

# 0.9.0 to 0.10.0

`horizon-lib` moves from 0.16.0 (`bd52627b24bf553de45eb639571c1127e44e7f1d`)
to 0.17.0 (`187b028a1c1ec6c8926a618a3ca435440d077ae7`).
`NodeCapability` gains `GitServer.NoSettings`, appended last:
`NodeCapability.[ … UsbDownlink.{ Ipv4Cidr } GitServer.NoSettings ]`.

Existing records decode unchanged; a cluster definition naming `GitServer` is
carried by any message holding a `HorizonDefinition` (`TestDefaults` carries
`Option<HorizonDefinition>`), so clients and daemon move together. `signal`,
`protos`, `datom-codec` and `ethos-zero` are unchanged, and
`src/generated/signal.rs` is byte-identical.

# 0.8.0 to 0.9.0

`horizon-lib` moves from 0.15.0 (`91abfd754fe09e296bb4de074204885b1e209b0d`)
to 0.16.0 (`bd52627b24bf553de45eb639571c1127e44e7f1d`).
`NodeCapability::OpenCodeTesting` carries a `UserName`, the server's owner,
where it carried `NoSettings`; `UserCapability` gains `StableCodex`:
`UserCapability.[ AgentStack CodexCandidate StableCodex ]`.

Any message holding a `HorizonDefinition` changes archive layout because a
`NodeCapability` payload changed and a `UserCapability` variant was added;
`TestDefaults` carries `Option<HorizonDefinition>`. Clients and daemon move
together. `signal`, `protos`, `datom-codec` and `ethos-zero` are unchanged, and
`src/generated/signal.rs` is byte-identical.

# 7.0.0 to 0.8.0

The numbering rule changes: the major number is only for a full rewrite, and
the past majors were wrong. A change to a stored or archived layout is a
subversion. This contract is now numbered in the 0.x scheme; 0.8.0 is the next
subversion after 7.0.0, and the next full rewrite is 1.0.0.

`horizon-lib` moves from 0.14.0 (`2e09ebbdc725f64fd9ee102b5758bab85be217e1`)
to 0.15.0 (`91abfd754fe09e296bb4de074204885b1e209b0d`). `UserPubKey` gains a
fourth position, `Vector<UserCapability>`, with
`UserCapability.[ AgentStack CodexCandidate ]`.

Any message holding a `HorizonDefinition` changes archive layout because
`UserPubKey` gained a position; `TestDefaults` carries
`Option<HorizonDefinition>`. Clients and daemon move together. `signal`,
`protos`, `datom-codec` and `ethos-zero` are unchanged, and
`src/generated/signal.rs` is byte-identical.

# 6.0.0 to 7.0.0

One step onto ethos-zero 16.0.0 and the 0.32.2 substrate:

- `signal` 3.0.2 (`8f9a0deb…`) to 8.0.0 (`f35460de930943ea1a8a972453ccf3a092a655b5`);
- `horizon-lib` 0.13.0 (`a3ddaf86…`) to 0.14.0 (`2e09ebbdc725f64fd9ee102b5758bab85be217e1`);
- `datom-codec` 0.31.0 (`09e2a9d5…`) to 0.32.2 (`4dff16b4f7412febc3b71aac8b49680cd20988cb`);
- `protos` 0.31.0 (`1febca78…`) to 0.32.2 (`15b41da8f2579e73ead59bc0c2b97529b8ac32d3`);
- `ethos-zero` 10.0.0 (`4bf73cae…`) to 16.0.0 (`c2653dd82adbdb1f1f2f654405c6620e0d06fd58`).

`src/generated/signal.rs` is regenerated (options as `std::option::Option`,
`#[rustfmt::skip]` on every item); no field, variant or archived layout of
this contract changed. What breaks is the trait identity under `datom`: every
type now implements datom-codec 0.32.2's `Datomizable` and `Composing`, and
`HorizonDefinition` comes from horizon-lib 0.14.0. A consumer (meta-signal-lojix,
Lojix) repins this revision together with signal 8.0.0, horizon-lib 0.14.0,
datom-codec and protos 0.32.2 in one step; `cargo tree -d` then shows one
datom-codec. The signal frame itself is whatever signal 8.0.0 defines; a
Nexus and its clients move together.

# 5.0.0 to 6.0.0

`horizon-lib` moves from 0.12.0 (`ee8d6f8d27eb6e200504807971ffdd26aaca7ed1`)
to 0.13.0 (`a3ddaf8685b920093a2328b85ba350a04e11477a`).

`HorizonDefinition` is part of this contract: `TestDefaults` carries
`Option<HorizonDefinition>` on the wire. Horizon 0.13.0 changes its archived
layout. `NodeCapability::TailnetClient` now carries a `SecretReference`,
`NodeCapability::TailnetController` carries the certificate authority and the
TLS certificate and key references, `NodeCapability` gains `UsbDownlink`, and
`RouterInterfaces` gains an eighth field, `country_code`.

Breaking: the rkyv archive of every message that can hold a
`HorizonDefinition` changes. Consumers repin and bump. `ethos-zero` and
`datom-codec` are unchanged, and `src/generated/signal.rs` is byte-identical.
The contract fixture now carries a router with a country, a tailnet
controller, a tailnet client and a USB downlink.

# 4.1.1 to 5.0.0

`DeploymentTerminalReason` gains `ClosureCopyFailed`.

A closure copy that fails was reported as `BuilderUnreachable`. A copy runs
`nix copy --substitute-on-destination --to <store-uri> <path>`: it engages no
builder at all, so that reason was false whatever the cause — an unreachable
target store, a refused signature, a full disk, or a malformed transport.
`BuilderUnreachable` and `SubstituterUnreachable` keep their own meanings; the
copy stage now names itself, exactly as `EvaluationFailed` and `BuildFailed`
were split out of `FlakeReferenceMalformed` in 3.0.0.

Breaking: the enum gains a variant, so its rkyv archive changes. Consumers
repin and add the arm.

# 4.1.0 to 4.1.1

A repin only. The producer chain settles on its final heads: `protos` 0.30.1
(`171b21f65337983ab624b7b906397a4f1f92c5a3`), `datom-codec` 0.26.3
(`627db67f2655efd9f786864009955005fd8ab2ad`), `ethos-zero` 8.0.1
(`de3d9928b156f2e1a92d060b7817af201abfdbef`), `signal` 3.0.2
(`8f9a0deb701cebbea518679548df4a795affc918`), `horizon-lib` 0.10.1
(`40d04d2504fee619e9b2b2564b8a769a3a9d6049`).

No type in this contract changed, and `src/generated/signal.rs` regenerates
byte-identical under ethos-zero 8.0.1 — `build.rs` asserts it on every build.
`Cargo.lock` carries exactly one revision of each of our crates.

Consumers repin the revision and change nothing else.

# 4.0.0 to 4.1.0

`horizon-lib` moves to 0.10.0 (`a56330451934d682ae15612acd49924356ec0205`),
which rehomes its free decode/compose/project functions onto traits. No type
in this crate changed, but `HorizonDefinition` is re-exported through the
contract, so a consumer pinning this crate pins that horizon-lib and must
import `DatomDecoding` and `Projecting` where it called the old free
functions. See horizon-rs UPGRADES.md.

# 2.0.0 to 4.0.0

Three changes to the ordinary contract, all breaking.

## A failure now carries the evidence a retry needs

`DeploymentFailure` gained a third field:

```
-DeploymentFailure.{ DeploymentFailureStage DeploymentTerminalReason }
+DeploymentFailure.{ DeploymentFailureStage DeploymentTerminalReason Option<FailureEvidence> }
+FailureEvidence.{ Option<FailedCommand> FailureDetail DetailTruncated }
+FailedCommand.{ CommandProgram Vector<CommandArgument> Option<ExitCode> }
```

Until now a failed deployment reported only a stage and one of eleven generic
reasons. The subprocess stderr that named the actual cause was captured by the
runtime and thrown away, so `Query.ByDeployment` could say *what phase* broke
and never *what broke*. `FailureEvidence` always carries the bounded tail of what the
failing stage printed, and `detail_truncated` says whether it was cut. The
command is optional within it: a stage that ran a subprocess names the
program, its arguments, and its exit code when the process reported one
(a process killed by a signal reports none); a stage that failed without
running one — a Horizon projection, an internal invariant — carries the
detail and no command.

The detail is redacted by the producer: any line containing a credential term
is dropped rather than stored. It is bounded, not a log stream — a Nexus does
not become a journal.

`None` for the whole evidence is the honest value where a failure carries
nothing to report — an admission rejection decided before any stage ran.

## The reason enum stops conflating evaluation with build

`DeploymentTerminalReason` gained `EvaluationFailed` and `BuildFailed`. Both
stages previously reported `FlakeReferenceMalformed`, which was true of
neither: a flake that evaluates and then fails to build is not a malformed
reference. Consumers matching exhaustively on the enum add the two arms.

## `CheckHostKeyMaterial` is gone

Removed from the request set, with `KeyMaterialChecked`,
`KeyMaterialCheckRejected`, `KeyMaterialQuery`, `KeyMaterialReport`,
`KeyMaterialMismatch`, `KeyMaterialConcern`, `MismatchValue`, `OperatorHint`,
`RejectedKeyMaterialCheck` and `KeyMaterialCheckRejectionReason`.

The verb was answered by a stub that reported an empty mismatch vector for
every node, and the adapter on the producing side discarded the vector
unconditionally. A security check that always answers "no mismatch" is worse
than no check: it can only mislead the operator who trusts it. Nothing
compared published key material against a live host, and nothing could — the
verb sat on the synchronous store-read path with no effect behind it.

If the check is wanted, it returns as an effect-backed verb with its own
pipeline stage, comparing the Horizon-published `ssh_public_key` and
Yggdrasil key view against the live host. That is a different contract from
this one, and it is not written yet.

# 1.2.0 to 2.0.0

The Signal frame type and its three kinds left this crate. `Signal<T>`,
`Signalizable`, `ByteViewable`, and `Restorable<T>` were defined here, in a
copy byte-identical to the one in every other contract crate. They now live
once, generically, in the `signal` repository, and this crate depends on it.

There is no compatibility path. A consumer changes its imports:

```rust
-use signal_lojix::{ByteViewable, Restorable, Signal, Signalizable, Query, Response};
+use signal::{ByteViewable, Restorable, Signal, Signalizable};
+use signal_lojix::{Query, Response};
```

and adds the dependency:

```toml
signal = { git = "https://github.com/LiGoldragon/signal", rev = "626e407be520a7a12f39b1d06c56ec423f3b3d09" }
```

The behavior is unchanged: the same rkyv bytes, the same validation on
restore. `Signalizable` and `Restorable<T>` are blanket implementations now,
so every contract type has them without the crate writing anything.
