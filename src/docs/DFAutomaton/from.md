Creates an DFA from initial state, list of finite states and transition function.

# Panics

Panics if `init_state` or elements of `accept_states` not a keys in `transitions` map.

# Examples

```compile_fail
use std::collections::{HashMap, HashSet};
use finite_automata::DFAutomaton;

let mut automaton = DFAutomaton::from(
    0,
    HashSet::from([0]),
    HashMap::from([
        (0, [2, 1, 1]), // Length of this array should be a 2
        (1, [3, 0]), // or length of this array should be a 3
      ],
));
```

```should_panic
use std::collections::{HashMap, HashSet};
use finite_automata::DFAutomaton;

let mut automaton = DFAutomaton::from(
    2, // State is not a key in transitions map.
    HashSet::from([0, 3]), // Array contains a state that is not a key in transitions map
    HashMap::from([
        (0, [2, 1]),
        (1, [3, 0]),
      ],
));
```
