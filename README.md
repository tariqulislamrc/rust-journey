# Rust for PHP Developers: exercises

Daily exercises for the free course at **https://rust.aoradev.xyz**: a 365-day path from PHP/Laravel to production Rust, two hours a day.

## Get started

1. Click **Use this template → Create a new repository** and name it `rust-journey` (public is recommended, since your daily commits become a portfolio).
2. Clone your copy:

   ```bash
   git clone git@github.com:YOUR_USERNAME/rust-journey.git ~/rust-journey
   ```

3. Follow the lesson for each day at https://rust.aoradev.xyz.

## Layout

```text
exercises/    one crate per coding day (day03, day04, ...), checked by tests
playground/   your own experiments and mini projects
```

## Doing an exercise

```bash
cd ~/rust-journey/exercises
cargo test -p day03
```

Each `src/lib.rs` has functions containing `todo!()` and tests at the bottom. Replace every `todo!()` until all tests pass. **Don't edit the tests.** They're the specification. Then run `cargo clippy -p day03`.

> Day 6 intentionally doesn't compile: fixing the ownership errors is the exercise.

New exercises are added every week. To get them into your copy:

```bash
git remote add upstream https://github.com/tariqulislamrc/rust-for-php-devs.git   # once
git pull upstream main --allow-unrelated-histories
```

## License

MIT
