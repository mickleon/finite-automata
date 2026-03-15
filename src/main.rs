use std::collections::{HashMap, HashSet};
use std::io;

use finite_automata::DFAutomaton;

const ALPHABET_LENGTH: usize = 2;

fn main() {
    #[rustfmt::skip]
    let mut automaton: DFAutomaton<ALPHABET_LENGTH> = DFAutomaton::from(
        0,
        HashSet::from([0]),
        HashMap::from([
            (0, [2, 1]),
            (1, [3, 0]),
            (2, [0, 3]),
            (3, [1, 2]),
        ],
    ));

    let mut input_string = String::new();
    loop {
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        let input_iter =
            input_string
                .trim()
                .chars()
                .map(|c| match c.to_digit(ALPHABET_LENGTH as u32) {
                    Some(symbol) => symbol as usize,
                    None => {
                        panic!("Undefined automaton input: \"{}\"", c);
                    }
                });
        println!("{}", automaton.is_accepting(input_iter));
        input_string.clear();
    }
}
