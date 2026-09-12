# Upgrades

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
