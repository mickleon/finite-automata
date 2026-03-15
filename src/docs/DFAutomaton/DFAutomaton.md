Struct for emulating of Deterministic Finite Automaton (DFA).

# Examples

```rust
use finite_automata::DFAutomaton;

let mut automaton: DFAutomaton<i32, char, 2> = DFAutomaton::from_arrays(
    0, // q_0 ∈ Q (initial state)
    &[0],  // F ⊆ Q (final states),
    &['0', '1'], // Σ (alphabet)
    &[  // ẟ: Q × Σ -> Q (transition map)
    // The order of the arrays is according to the order of the `alphabet`
    // | q | ẟ(q, '0') | ẟ(q, '1') |
        (0, [2,          1]),
        (1, [3,          0]),
        (2, [0,          3]),
        (3, [1,          2]),
    ]
);

assert!(automaton.run(['0', '1', '1', '0']));
assert!(!automaton.run(['1', '1', '1', '0']));

automaton.reset();
assert_eq!(automaton.get_current_state(), 0);

automaton.step('1'); // current_state = ẟ(0, '1');
assert_eq!(automaton.get_current_state(), 1);
assert!(!automaton.is_accepting());
```

Length of arrays in transitions map is constant and their order is according to alphabet. This code can not be compiled:

```compile_fail
use finite_automata::DFAutomaton;

let mut automaton = DFAutomaton::from_arrays(
    0, &[0], &['0', '1'], // `alphabet` length is 2
    &[
        (0, [0, 1, 1]), // Lengh of array shoud equal length of `alphabet`
        (1, [1, 0]),
    ]
);
```
