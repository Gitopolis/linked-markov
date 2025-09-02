//! # linked-markov
//!
//! A minimal, thread-safe Markov chain implementation using reference-counted steps and weighted transitions.
//!
//! ## Features
//! - Generic over state type `T` (must be `Eq + Copy + Hash + Debug`)
//! - Weighted transitions between states
//! - Deterministic and mutable walks
//!
//! ## Example
//! ```rust
//! use linked_markov::{Step, ToStep, walk};
//! use std::sync::Arc;
//!
//! let step_false: ToStep<bool> = Arc::new(Step::new(false));
//! let step_true: ToStep<bool> = Arc::new(Step::new(true));
//! step_false.insert_transition(step_true.clone(), 3);
//! step_false.insert_transition(step_false.clone(), 1);
//! step_true.insert_transition(step_false.clone(), 3);
//! step_true.insert_transition(step_true.clone(), 1);
//! let path = walk(step_false, 100);
//! assert_eq!(path.len(), 100);
//! ```
mod step;
pub use step::{Step, ToStep, mut_walk, walk};

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn two_state_walk() {
        let step_false: ToStep<bool> = Arc::new(Step::new(false));
        let step_true: ToStep<bool> = Arc::new(Step::new(true));

        step_false.insert_transition(step_true.clone(), 3);
        step_false.insert_transition(step_false.clone(), 1);
        step_true.insert_transition(step_false.clone(), 3);
        step_true.insert_transition(step_true.clone(), 1);
        let path = walk(step_false, 100);
        assert_eq!(path.len(), 100);
        assert!(path.contains(&false));
        assert!(path.contains(&true));
    }

    #[test]
    fn two_state_mut_walk() {
        let step_false: ToStep<bool> = Arc::new(Step::new(false));
        let step_true: ToStep<bool> = Arc::new(Step::new(true));

        step_false.insert_transition(step_true.clone(), 1);
        step_false.insert_transition(step_false.clone(), 1);
        step_true.insert_transition(step_false.clone(), 1);
        step_true.insert_transition(step_true.clone(), 1);
        let path = mut_walk(step_false.clone(), 100, |current, next| {
            current
                .transitions
                .write()
                .unwrap()
                .entry(next)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            Ok(())
        })
        .unwrap();
        let step_true_count = step_true
            .transitions
            .read()
            .unwrap()
            .values()
            .sum::<usize>();
        let step_false_count = step_false
            .transitions
            .read()
            .unwrap()
            .values()
            .sum::<usize>();
        assert_eq!(path.len(), 100);
        assert!(path.contains(&false));
        assert!(path.contains(&true));
        assert_eq!(step_true_count + step_false_count, 103);
    }
}
