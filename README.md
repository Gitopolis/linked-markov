# linked-markov

[![License: CC0 1.0](https://img.shields.io/badge/license-CC0%201.0-lightgrey.svg)](LICENSE)
[![CI](https://github.com/Gitopolis/linked-markov/actions/workflows/ci.yml/badge.svg)](https://github.com/Gitopolis/linked-markov/actions)

A minimal, thread-safe Markov chain implementation using reference-counted steps and weighted transitions.

## Features

- Generic over state type `T` (must be `Eq + Copy + Hash + Debug` and `Send + Sync`)
- Weighted transitions between states
- Non-mutable (`walk`) and mutable (`mut_walk`) traversal utilities

## Quick start

Add the crate (if published) or use the local path in your workspace. Example usage below uses the library directly.

```mermaid
---
title: Two-state non-deterministic chain
---
stateDiagram-v2
  direction LR
  False --> True: 75%
  True --> False: 75%
```

```rust
use linked_markov::{Step, ToStep, walk, mut_walk};
use std::sync::Arc;

// Create two states and wire weighted transitions between them.
let step_false: ToStep<bool> = Arc::new(Step::new(false));
let step_true: ToStep<bool> = Arc::new(Step::new(true));

step_false.insert_transition(step_true.clone(), 3);
step_false.insert_transition(step_false.clone(), 1);
step_true.insert_transition(step_false.clone(), 3);
step_true.insert_transition(step_true.clone(), 1);

let path = walk(step_false.clone(), 100);
assert_eq!(path.len(), 100);
```

## Mutable walk example

`mut_walk` accepts a callback that's called for every successful transition. This allows you to mutate transition weights or collect statistics.

```mermaid
---
title: Two-state non-deterministic chain
---
stateDiagram-v2
  direction LR
  False --> True: 50%
  True --> False: 50%
```

```rust
use std::sync::Arc;
use linked_markov::{Step, ToStep, mut_walk};

let step_false: ToStep<bool> = Arc::new(Step::new(false));
let step_true: ToStep<bool> = Arc::new(Step::new(true));

step_false.insert_transition(step_true.clone(), 1);
step_false.insert_transition(step_false.clone(), 1);
step_true.insert_transition(step_false.clone(), 1);
step_true.insert_transition(step_true.clone(), 1);

let path = mut_walk(step_false.clone(), 100, |current, next| {
  current
    .transitions
    .lock()
    .unwrap()
    .entry(next)
    .and_modify(|e| *e += 1)
    .or_insert(1);
  Ok(())
}).unwrap();

let step_true_count = step_true.transitions.lock().unwrap().values().sum::<usize>();
let step_false_count = step_false.transitions.lock().unwrap().values().sum::<usize>();
assert_eq!(path.len(), 100);
assert_eq!(step_true_count + step_false_count, 103);
```

## Public API (summary)

- `Step<T>`: Node holding a `state` and `transitions`.
- `ToStep<T>`: `Arc<Step<T>>` — shared pointer to a step.
- `Step::new(state: T) -> Step<T>`: create a new step.
- `Step::insert_transition(&self, to_step: ToStep<T>, weight: usize)`: add or update a weighted transition.
- `Step::next(&self) -> Option<ToStep<T>>`: choose the next step randomly by weights.
- `walk(start: ToStep<T>, steps: usize) -> Vec<T>`: traverse and return visited states.
- `mut_walk(start: ToStep<T>, steps: usize, apply: F) -> Result<Vec<T>, Box<dyn std::error::Error>>`: traverse while calling `apply(current, next)` for every transition.

## Docs & tests

Generate API docs:

```bash
cargo doc --open
```

Run tests:

```bash
cargo test
```

## License

This project is dedicated to the public domain under the Creative Commons
CC0 1.0 Universal public domain dedication. See the repository `LICENSE`
file for the full legal text.

Short summary: the author has waived all copyright and related rights to
the extent possible under law. See `LICENSE` for details.

## Contributing

Contributions are welcome. By submitting a pull request or other
contribution you agree to license your contribution under the same
CC0 1.0 Universal dedication used by this repository. In short, you
waive copyright and related rights in your contribution to the extent
possible under law.

Quick dev commands:

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
