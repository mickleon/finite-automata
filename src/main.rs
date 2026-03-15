use std::io;

use finite_automata::DFAutomaton;

fn main() {
    let init_state = 0;
    let accept_states = [0];
    let alphabet = ['0', '1'];
    #[rustfmt::skip]
    let transitions = [
        (0, [2, 1]),
        (1, [3, 0]),
        (2, [0, 3]),
        (3, [1, 2])
    ];
    let mut automaton =
        DFAutomaton::from_arrays(init_state, &accept_states, &alphabet, &transitions);

    let mut input_string = String::new();
    loop {
        io::stdin()
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
