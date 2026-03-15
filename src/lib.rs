use std::collections::{HashMap, HashSet};

#[doc = include_str!("docs/DFAutomaton/DFAutomaton.md")]
pub struct DFAutomaton<const ALPHABET_LEN: usize> {
    init_state: u32,
    accept_states: HashSet<u32>,
    transitions: HashMap<u32, [u32; ALPHABET_LEN]>,
    current_state: u32,
}

impl<const ALPHABET_LEN: usize> DFAutomaton<ALPHABET_LEN> {
    const ALPHABET_LEN: usize = ALPHABET_LEN;

    #[doc = include_str!("docs/DFAutomaton/from.md")]
    pub fn from(
        init_state: u32,
        accept_states: HashSet<u32>,
        transitions: HashMap<u32, [u32; ALPHABET_LEN]>,
    ) -> Self {
        if !(transitions.contains_key(&init_state)) {
            panic!("Initial state must be as a key in the `next_states` map");
        }

        for state in accept_states.iter() {
            if !transitions.contains_key(state) {
                panic!("All finite states must be as a key in the `next_states` map");
            }
        }

        Self {
            init_state,
            accept_states,
            transitions,
            current_state: init_state,
        }
    }

    /// Puts the DFA to init state.
    pub fn put_to_init_state(&mut self) {
        self.current_state = self.init_state;
    }

    // Returns a current state.
    pub fn get_current_state(&self) -> u32 {
        self.current_state
    }

    /// Gets an input symbol and puts the DFA in the appropriate state according to the transitions map, depending on the current state.
    ///
    /// # Panics
    ///
    /// Panics if the transition for symbol not defined in transitions map.
    ///
    /// Or if elements of `input` are greater or equal of the `ALPHABET_LEN`.
    pub fn transition(&mut self, symbol: usize) {
        if symbol >= Self::ALPHABET_LEN {
            panic!("Undefined input symbol: \"{}\"", symbol);
        }
        let new_state = match self.transitions.get(&self.current_state) {
            Some(states) => states[symbol],
            None => {
                panic!("Undefined next state for \"{}\"", self.current_state);
            }
        };

        self.current_state = new_state;
    }

    /// Gets a iterator `input` of DFA symbols and returns `true` if the DFA accepts this string, `false` otherwise.
    ///
    /// Resets the current state to the initial state before computing.
    ///
    /// # Panics
    ///
    /// Panics if the row elements cause the `transition()` function to panic.
    pub fn is_accepting<I>(&mut self, input: I) -> bool
    where
        I: IntoIterator<Item = usize>,
    {
        self.put_to_init_state();
        for symbol in input {
            self.transition(symbol);
        }
        self.accept_states.contains(&self.current_state)
    }
}
