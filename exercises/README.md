# Exercises

One crate per coding day. Each `src/lib.rs` contains functions with `todo!()` and a set of tests at the bottom.

**Your job:** replace every `todo!()` with a working implementation until all tests pass.

```bash
cd ~/rust-journey/exercises
cargo test -p day03              # run one day's tests
cargo test -p day03 -- --nocapture   # also show println! output
cargo test -p day03 seconds      # run only tests whose name contains "seconds"
```

Rules:

1. **Don't edit the tests.** They're the specification.
2. Work one function at a time. A test fails with `not yet implemented` until you replace its `todo!()`.
3. When everything passes, run `cargo clippy -p dayNN` and fix what it suggests.
4. Stuck for more than 30 minutes? Check the hint in the lesson first, then ask Claude.
