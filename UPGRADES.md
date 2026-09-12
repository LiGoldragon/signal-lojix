# Upgrades

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
