# Contributing

Thanks for helping. All changes go in through a pull request; nobody pushes to `main` directly.

## Steps

1. Fork the repo and make a branch.
2. Make your change. Keep it to one thing.
3. Run `cargo fmt`, `cargo clippy -- -D warnings` and `cargo test`. All three must pass.
4. Open a pull request saying what it does and how you checked it.

## Rules for physics code

- Every new quantity needs a test against a known value: a textbook result, or a RASPA3 run on the same input (say which example, how many cycles, and the RASPA3 version).
- State units in names or doc comments (`energy_kelvin`, `length_angstrom`).
- If you port logic from RASPA3, cite the file it came from in a comment. RASPA3 is MIT licensed; keep its notice (see NOTICE).

## Requests

Not coding? Open an issue with the feature request or bug form. Questions go in Discussions.
