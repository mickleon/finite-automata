use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

/// Error that can occur when applying a transition in [`DFAutomaton::step`].
#[derive(Debug, Clone)]
pub enum StepError<C> {
    /// The input symbol is not in the `alphabet` passed to [`DFAutomaton::from_arrays`].
    UndefinedSymbol(C),
}

impl<C: fmt::Display> fmt::Display for StepError<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepError::UndefinedSymbol(symbol) => {
                write!(f, "Undefined input symbol: \"{}\"", symbol)
            }
        }
    }
}

#[doc = include_str!("docs/DFAutomaton/DFAutomaton.md")]
#[derive(Debug, Clone)]
pub struct DFAutomaton<S, C, const N: usize> {
    init_state: usize,
    accept_states: HashSet<usize>,
    transitions: Vec<[usize; N]>,
    current_state: usize,

    states: Vec<S>,
    symbols: HashMap<C, usize>,
}

impl<S, C, const N: usize> DFAutomaton<S, C, N>
where
    S: Eq + Hash + Copy + fmt::Display,
    C: Eq + Hash + Copy + fmt::Display,
{
    /// Creates an DFA from initial state, list of accept states, alphabet and transition function.
    ///
    /// # Panics
    ///
    /// Panics if
    /// - `init_state` not in `transitions`
    /// - elements of `accept_states` not in `transitions`
    /// - there is duplicate state in `transitions`
    /// - there is state in transition that not defined as a source state.
    pub fn from_arrays(
        init_state: S,
        accept_states: &[S],
        alphabet: &[C; N],
        transitions: &[(S, [S; N])],
    ) -> Self {
        // map Symbol -> Index
        let symbols_map: HashMap<C, usize> = alphabet
            .iter()
            .enumerate()
            .map(|(index, &symbol)| (symbol, index))
            .collect();

        // temporary map State -> Index
        let mut state_indices: HashMap<S, usize> = HashMap::new();
        // vector of states
        let mut states = Vec::new();
        for (index, (state, _)) in transitions.iter().enumerate() {
            if state_indices.insert(*state, index).is_some() {
                panic!("Duplicate state in transitions map: \"{}\"", state);
            }
            states.push(*state);
        }

        // init state index
        let init_state_index: usize = *state_indices.get(&init_state).unwrap_or_else(|| {
            panic!(
                "Initial state \"{}\" not defined in transitions map",
                init_state
            )
        });

        // accept states set
        let accept_states_set: HashSet<usize> = accept_states
            .iter()
            .map(|state| {
                *state_indices.get(state).unwrap_or_else(|| {
                    panic!("Accept state \"{}\" not in transitions map", state);
                })
            })
            .collect();

        // transitions table
        let mut transitions_vec: Vec<[usize; N]> = Vec::with_capacity(states.len());
        for (source_state, dest_states) in transitions.iter() {
            let mut row = [0; N];
            for (index, dest_state) in dest_states.iter().enumerate() {
                let dest_index = *state_indices.get(dest_state).unwrap_or_else(|| {
                    panic!(
                        "State \"{}\" used in transition from \"{}\" but not defined as a source state",
                        dest_state, source_state
                    );
                });
                row[index] = dest_index;
            }
            transitions_vec.push(row);
        }

        Self {
            init_state: init_state_index,
            accept_states: accept_states_set,
            transitions: transitions_vec,
            current_state: init_state_index,

            states,
            symbols: symbols_map,
        }
    }

    /// Puts the DFA to init state.
    pub fn reset(&mut self) {
        self.current_state = self.init_state;
    }

    /// Returns a current state.
    pub fn get_current_state(&self) -> S {
        self.states[self.current_state]
    }

    /// Returns `true` if the current state of DFA is accepting, `false` otherwise.
    pub fn is_accepting(&self) -> bool {
        self.accept_states.contains(&self.current_state)
    }

    /// Gets an input symbol and puts the DFA in the appropriate state according to the transitions map, depending on the current state.
    ///
    /// # Errors
    ///
    /// Returns [`StepError::UndefinedSymbol`] if `symbol` is not in `alphabet` passed to [`DFAutomaton::from_arrays`].
    pub fn step(&mut self, symbol: C) -> Result<(), StepError<C>> {
        let symbol_index: usize = *self
            .symbols
            .get(&symbol)
            .ok_or(StepError::UndefinedSymbol(symbol))?;

        self.current_state = self.transitions[self.current_state][symbol_index];
        Ok(())
    }

    /// Gets an iterator `input` of DFA symbols and returns `Ok(true)` if the DFA accepts this string, `Ok(false)` otherwise.
    ///
    /// Resets the current state to the initial state before computing.
    ///
    /// # Errors
    ///
    /// Returns [`StepError::UndefinedSymbol`] if the string contains a symbol that not in `alphabet` passed in [`DFAutomaton::from_arrays`].
    pub fn run<I>(&mut self, input: I) -> Result<bool, StepError<C>>
    where
        I: IntoIterator<Item = C>,
    {
        self.reset();
        for symbol in input {
            self.step(symbol)?;
        }
        Ok(self.is_accepting())
    }
}
