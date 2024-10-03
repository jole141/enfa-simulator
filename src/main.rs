mod state;
mod input_parsing;
mod state_helper;

use crate::input_parsing::*;
use crate::state_helper::*;

fn main() {
    let input_lines = read_input_lines();
    let (machine_inputs, states, starting_state_name) = organized_inputs_into_data(&input_lines);
    let mut output = String::new();

    for machine_input in machine_inputs {
        let state = get_state_instance(starting_state_name, &states);
        let mut states = Vec::new();
        states.push(state.clone());
        output.push_str(state.borrow().name);
        let epsilon_transitions = state.borrow().check_for_epsilon_transitions();

        if !epsilon_transitions.is_empty() {
            for epsilon_state in epsilon_transitions {
                states.push(epsilon_state.clone());
                output.push_str(",");
                output.push_str(epsilon_state.borrow().name);
            }
        }

        for symbol in machine_input {
            if output.ends_with('|') {
                output.push_str("#");
            } else if output.ends_with(',') {
                output.pop();
            }
            output.push_str("|");
            let states_to_be_processed = states.clone();
            states.clear();
            for state in states_to_be_processed {
                let transition_states = state.borrow().check_for_symbol_transition(symbol);
                for transition_state in transition_states {
                    states.push(transition_state.clone());
                    output.push_str(transition_state.borrow().name);
                    output.push_str(",");
                    let epsilon_transitions = transition_state.borrow().check_for_epsilon_transitions();

                    if !epsilon_transitions.is_empty() {
                        for epsilon_state in epsilon_transitions {
                            states.push(epsilon_state.clone());
                            output.push_str(epsilon_state.borrow().name);
                            output.push_str(",");
                        }
                    }
                }
            }
        }
        if output.ends_with('|') {
            output.push_str("#");
        } else if output.ends_with(',') {
            output.pop();
        }
        println!("{}", output);
        output.clear();
    }
}