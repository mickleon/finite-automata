Creates an DFA from initial state, list of finite states and transition function.

# Panics

Panics if following elements not a keys in `transitions` map:

- `init_state`

  ```should_panic
  # use std::collections::{HashMap, HashSet};
  # use finite_automata::DFAutomaton;
  let mut automaton = DFAutomaton::from(
      2, // State is not a key in transitions map.
      HashSet::from([0]),
      HashMap::from([
          (0, [0, 1]),
          (1, [1, 0]),
      ],
  ));
  ```

- elements of `accept_states`;

  ```should_panic
  # use std::collections::{HashMap, HashSet};
  # use finite_automata::DFAutomaton;
  let mut automaton = DFAutomaton::from(
      0,
      HashSet::from([0, 3]), // Array contains a state that is not a key in transitions map
      HashMap::from([
          (0, [0, 1]),
          (1, [1, 0]),
      ],
  ));
  ```

- elements of array values in `transitions` map.

  ```should_panic
  # use std::collections::{HashMap, HashSet};
  # use finite_automata::DFAutomaton;
  let mut automaton = DFAutomaton::from(
      0,
      HashSet::from([0]),
      HashMap::from([
          (0, [0, 2]), // 2 is not a key in transitions map
          (1, [1, 0]),
      ],
  ));
  ```
