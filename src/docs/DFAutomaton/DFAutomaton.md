Struct for emulating a Deterministic Finite Automaton (DFA).

# Examples

```rust
use finite_automata::DFAutomaton;

let init_state = 'A';        // q_0 ∈ Q
let accept_states = ['A'];   // F ⊆ Q
let alphabet = [0, 1]; // Σ
let transitions = [        // ẟ: Q × Σ -> Q
// The order of the arrays is according to the order of the `alphabet`
//  | q   | ẟ(q, 0) | ẟ(q, 1) |
    ('A', ['C',      'B']), // Accepts words with an even number of
    ('B', ['D',      'A']), // each of the symbols 1 and 0
    ('C', ['A',      'D']),
    ('D', ['B',      'C']),
];

let mut automaton: DFAutomaton<char, u32> = 
    DFAutomaton::from_arrays(
        init_state,
        &accept_states,
        &alphabet,
        &transitions
    );

assert!(automaton.run([0, 1, 1, 0]).unwrap());
assert!(!automaton.run([1, 1, 1, 0]).unwrap());

automaton.reset();
assert_eq!(automaton.get_current_state(), 'A');

automaton.step(1).unwrap(); // current_state = ẟ('A', 1);
assert_eq!(automaton.get_current_state(), 'B');
assert!(!automaton.is_accepting());
```

Order of elements of arrays in transitions map is according to alphabet.

```compile_fail
# use finite_automata::DFAutomaton;
let mut automaton = DFAutomaton::from_arrays(
    0, &[0], &['0', '1'], // `alphabet` length is 2
    &[
        (0, [0, 1, 1]), // Lengh of array shoud equal to length of `alphabet`
        (1, [1, 0]),
    ]
);
```
