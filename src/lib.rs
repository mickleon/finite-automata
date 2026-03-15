use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

#[doc = include_str!("docs/DFAutomaton/DFAutomaton.md")]
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
    S: Eq + Hash + Copy + Display,
    C: Eq + Hash + Copy + Display,
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
        let symbols_map: HashMap<C, usize> = alphabet
            .iter()
            .enumerate()
            .map(|(index, &symbol)| (symbol, index))
            .collect();

        let mut state_indices = HashMap::new();
        let mut states = Vec::new();

        for (index, (state, _)) in transitions.iter().enumerate() {
            if state_indices.insert(*state, index).is_some() {
                panic!("Duplicate state in transitions map: \"{}\"", state);
            }
            states.push(*state);
        }

        let init_state_index = *state_indices.get(&init_state).unwrap_or_else(|| {
            panic!(
                "Initial state \"{}\" not defined in transitions map",
                init_state
            )
        });

        let accept_states_set = accept_states
            .iter()
            .map(|state| {
                *state_indices.get(state).unwrap_or_else(|| {
                    panic!("Accept state \"{}\" not in transitions map", state);
                })
            })
            .collect();

        let mut transitions_vec = Vec::with_capacity(states.len());

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
    /// # Panics
    ///
    /// Panics
    /// - if the transition for `symbol` not defined in transitions map
    /// - `symbol` didn't passed in `alphabet` in [`Self::from_arrays`].
    pub fn step(&mut self, symbol: C) {
        let symbol_index = *self.symbols.get(&symbol).unwrap_or_else(|| {
            panic!("Undefined input symbol: \"{}\"", symbol);
        });

        let states = self.transitions.get(self.current_state).unwrap_or_else(|| {
            panic!(
                "Undefined next state for \"{}\"",
                self.states[self.current_state]
            );
        });

        self.current_state = states[symbol_index];
    }

    /// Gets a iterator `input` of DFA symbols and returns `true` if the DFA accepts this string, `false` otherwise.
    ///
    /// Resets the current state to the initial state before computing.
    ///
    /// # Panics
    ///
    /// Panics if the input symbols cause the [`Self::step`] function to panic.
    pub fn run<I>(&mut self, input: I) -> bool
    where
        I: IntoIterator<Item = C>,
    {
        self.reset();
        for symbol in input {
            self.step(symbol);
        }
        self.is_accepting()
    }
}
