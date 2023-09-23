This repo showcases an unfortunate interaction with feature unification and `-Zbindeps`.

A common dependency (`featuredep`) is depended upon from two crates. First, `bindep`, which does *not* enable feature `foo`, and second, `incorporator`, which does. `incorporator` also depends on `bindep` as a bin dependency.

When `bindep` is built without specifying a target, `foo` is enabled for it. When it is built for *any target in particular*, same or different, the features are separated as normal.

You can see for yourself:

```sh
cargo run -p check-host-target
cargo run -p check-diff-target
cargo run -p check-explicit-target
```

`check-diff-target` is only implemented for x64 Windows or Linux, and cannot be used with WSL1.

`unrelated` is added just for more feature unification noise, so this repo can be used in the future.
