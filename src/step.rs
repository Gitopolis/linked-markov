use rand::prelude::*;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    sync::{Arc, RwLock},
};

/// A reference-counted pointer to a `Step` in the Markov chain.
pub type ToStep<T> = Arc<Step<T>>;

/// A node in the Markov chain, holding a state and weighted transitions to other steps.
#[derive(Default)]
pub struct Step<T: Eq + Copy + Hash + Debug + Send + Sync> {
    /// The state value for this step.
    pub state: T,
    /// Outgoing transitions and their weights.
    pub transitions: RwLock<HashMap<ToStep<T>, usize>>,
}

impl<T> Clone for Step<T>
where
    T: Eq + Copy + Hash + Debug + Send + Sync,
{
    fn clone(&self) -> Self {
        #[allow(clippy::mutable_key_type)]
        let transitions = self.transitions.read().unwrap().clone();
        Step {
            state: self.state,
            transitions: RwLock::new(transitions),
        }
    }
}

impl<T> Debug for Step<T>
where
    T: Eq + Copy + Hash + Debug + Send + Sync,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Step {{ state: {:?} }}", self.state)
    }
}

impl<T> Hash for Step<T>
where
    T: Eq + Copy + Hash + Debug + Send + Sync,
{
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.state.hash(hasher);
    }
}

impl<T> PartialEq for Step<T>
where
    T: Eq + Copy + Hash + Debug + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<T> Eq for Step<T> where T: Eq + Copy + Hash + Debug + Send + Sync {}

impl<T> Step<T>
where
    T: Eq + Copy + Hash + Debug + Send + Sync,
{
    /// Create a new `Step` with the given state.
    pub fn new(state: T) -> Self {
        Step {
            state,
            transitions: RwLock::new(HashMap::new()),
        }
    }

    /// Add or update a transition to another step with a given weight.
    pub fn insert_transition(&self, to_step: ToStep<T>, weight: usize) {
        self.transitions.write().unwrap().insert(to_step, weight);
    }

    /// Randomly select the next step based on transition weights.
    pub fn next(&self) -> Option<ToStep<T>> {
        let mut rng = rand::rng();
        let transitions = self.transitions.read().unwrap();
        if transitions.is_empty() {
            return None;
        }
        let total: usize = transitions.values().sum();
        if total == 0 {
            return None;
        }
        let roll = rng.random_range(0..total);
        let mut cumulative = 0;
        transitions.iter().find_map(|(to_step, &weight)| {
            cumulative += weight;
            if roll < cumulative {
                Some(Arc::clone(to_step))
            } else {
                None
            }
        })
    }
}

/// Walk the Markov chain for a fixed number of steps, returning the visited states.
pub fn walk<T>(start: ToStep<T>, steps: usize) -> Vec<T>
where
    T: Eq + Copy + Hash + Debug + Send + Sync,
{
    let mut current = start;
    let mut path = vec![current.state];
    for _ in 1..steps {
        if let Some(next) = current.next() {
            path.push(next.state);
            current = next;
        } else {
            break;
        }
    }
    path
}

/// Walk the Markov chain for a fixed number of steps, applying a function to each transition.
///
/// The `apply` function is called with the current and next step, and can mutate the chain or collect data.
/// # Examples:
/// ```
/// use linked_markov::{Step, ToStep, mut_walk};
/// use std::sync::Arc;
///
/// let step_false: ToStep<bool> = Arc::new(Step::new(false));
/// let step_true: ToStep<bool> = Arc::new(Step::new(true));
/// step_false.insert_transition(step_true.clone(), 3);
/// step_false.insert_transition(step_false.clone(), 1);
/// step_true.insert_transition(step_false.clone(), 3);
/// step_true.insert_transition(step_true.clone(), 1);
/// let path = mut_walk(step_false, 100, |current, next| {
///     current
///         .transitions
///         .write()
///         .unwrap()
///         .entry(next)
///         .and_modify(|e| *e += 1)
///         .or_insert(1);
///     Ok(())
/// })
/// .unwrap();
/// ```
pub fn mut_walk<T, F>(start: ToStep<T>, steps: usize, apply: F) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Eq + Copy + Hash + Debug + Send + Sync,
    F: Fn(ToStep<T>, ToStep<T>) -> Result<(), Box<dyn Error>>,
{
    let mut current = start;
    let mut path = vec![current.state];
    for _ in 1..steps {
        if let Some(next) = current.next() {
            apply(current.clone(), next.clone())?;
            path.push(current.state);
            current = next;
        } else {
            break;
        }
    }
    Ok(path)
}
