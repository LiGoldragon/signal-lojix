# Upgrades

# 2.0.0 to 3.0.0

Three changes to the ordinary contract, all breaking.

## A failure now carries the evidence a retry needs

`DeploymentFailure` gained a third field:

```
-DeploymentFailure.{ DeploymentFailureStage DeploymentTerminalReason }
+DeploymentFailure.{ DeploymentFailureStage DeploymentTerminalReason Option<FailureEvidence> }
+FailureEvidence.{ CommandProgram Vector<CommandArgument> Option<ExitCode> FailureDetail DetailTruncated }
```

Until now a failed deployment reported only a stage and one of eleven generic
reasons. The subprocess stderr that named the actual cause was captured by the
runtime and thrown away, so `Query.ByDeployment` could say *what phase* broke
and never *what broke*. `FailureEvidence` carries the failed command's program
and arguments, its exit code when the process reported one, and the bounded
tail of its stderr. `detail_truncated` says whether the detail was cut.

The detail is redacted by the producer: any line containing a credential term
is dropped rather than stored. It is bounded, not a log stream — a Nexus does
not become a journal.

`None` is the honest value where a failure has no subprocess behind it (an
admission rejection, an internal invariant).

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
