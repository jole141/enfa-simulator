use std::io;
use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::state_helper::*;

pub fn read_input_lines() -> Vec<String> {
    let mut input_lines: Vec<String> = Vec::new(); // input symbols for automate
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Read line error");
    
        let input = input.trim().to_string();

        // handle empty string as end of input
        if input.is_empty() {
            break;
        }

        input_lines.push(input)
    }

    input_lines
}

pub fn organized_inputs_into_data<'a>(input_data: &'a Vec<String>) -> (Vec<Vec<&'a str>>, Vec<Rc<RefCell<State>>>, &str) {
    let mut machine_inputs: Vec<Vec<&'a str>> = Vec::new();
    let mut states: Vec<Rc<RefCell<State>>> = Vec::new();
    let mut starting_state_name: &str = "";

    let mut helper_counter = 0;
    
    for line in input_data {
        let line_str: &str = line.as_str();
        if helper_counter == 0 {
            for input_part in line_str.split("|") {
                machine_inputs.push(
                    input_part
                        .split(",")
                        .collect()
                );
            }
        } else if helper_counter == 1 {
            for input_part in line_str.split(",") {
                states.push(State::new(input_part));
            }
        } else if helper_counter == 4 {
            starting_state_name = line_str;
        } else if helper_counter > 4 {
            // e.g. s3,a->stanje2
            let splited_line: Vec<&str> = line_str.split("->").collect();
            let state_with_symbol: Vec<&str> = splited_line[0]
                .split(",")
                .collect(); // [state, symbol]

            let transition_states: Vec<&str> = splited_line[1]
                .split(",")
                .collect(); // list of states

            let state: Rc<RefCell<State>> = get_state_instance(state_with_symbol[0], &states);
            let transition_states = get_transition_states(&transition_states, &states);
            if !transition_states.is_empty() {
                if state_with_symbol[1] == "$" {
                    state.borrow_mut().epsilon_transitions.insert(state_with_symbol[1], transition_states);
                } else {
                    state.borrow_mut().transitions.insert(state_with_symbol[1], transition_states);
                }
            }
        }
        helper_counter += 1;
    }
    
    (machine_inputs, states, starting_state_name)
}