Struct for emulating of Deterministic Finite Automaton (DFA).

`u32` is used as a state type and `usize` as a symbol type.

# Examples

```rust
use std::collections::{HashMap, HashSet};
use finite_automata::DFAutomaton;

let mut automaton/*: DFAutomaton<2>*/ = DFAutomaton::from( // There is 2 symbols in alphabet
    0, // q_0 ∈ Q (initial state)
    HashSet::from([0]), // F ⊆ Q (final states)
    HashMap::from([ // ẟ: Q × Σ -> Q transition function
      // | q | ẟ(q, 0) | ẟ(q, 1) |
         (0,  [2,        1]),
         (1,  [3,        0]),
         (2,  [0,        3]),
         (3,  [1,        2]),
    ],
));

assert!(automaton.is_accepting([0, 1, 1, 0]));
assert!(!automaton.is_accepting([1, 1, 1, 0]));

automaton.put_to_init_state();
assert_eq!(automaton.get_current_state(), 0);

automaton.transition(1); // current_state = ẟ(0, 1);
assert_eq!(automaton.get_current_state(), 1);
```

Length of arrays in `transitions` map is constant (`ALPHABET_LEN`). This code can not be compiled:

```compile_fail
use std::collections::{HashMap, HashSet};
use finite_automata::DFAutomaton;

let mut automaton = DFAutomaton::from(
    0,
    HashSet::from([0]),
    HashMap::from([
        (0, [0, 1, 1]), // Length of this array should be a 2
        (1, [1, 0]), // or length of this array should be a 3
    ],
));
```
