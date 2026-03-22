use finite_automata::DFAutomaton;

fn main() {
    let init_state = 'A'; // q_0 ∈ Q
    let accept_states = ['A']; // F ⊆ Q
    let alphabet = ['0', '1']; // Σ
    let transitions = [
        // ẟ: Q × Σ -> Q
        // The order of the arrays is according to the order of the `alphabet`
        //  | q | ẟ(q, 0) | ẟ(q, 1) |
        ('A', ['C', 'B']),
        ('B', ['D', 'A']),
        ('C', ['A', 'D']),
        ('D', ['B', 'C']),
    ];
    let mut automaton =
        DFAutomaton::from_arrays(init_state, &accept_states, &alphabet, &transitions);

    let mut input_string = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        let input_iter = input_string.trim().chars();
        match automaton.run(input_iter) {
            Ok(accepted) => println!("{}", accepted),
            Err(e) => println!("Error: {}", e),
        }
        input_string.clear();
    }
}
