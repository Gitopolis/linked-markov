# Contributing to linked-markov

Thank you for your interest in contributing. By contributing code, tests,
documentation, or other content to this repository you agree to license
that contribution under the same CC0 1.0 Universal dedication used by
this project. If you can't accept CC0 for your contribution, please open
an issue to discuss alternative arrangements.

How to contribute

- Fork the repository and create a feature branch.
- Add tests for new behavior where appropriate.
- Keep changes focused and well-documented.
- Run `cargo fmt` and `cargo clippy` before submitting a PR.

Development commands

```bash
# run tests
cargo test

# build docs locally
cargo doc --no-deps --open

# format
cargo fmt

# run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

Reporting security issues

If you find a security issue, please do not open a public issue. Instead
contact the repository maintainers privately.
