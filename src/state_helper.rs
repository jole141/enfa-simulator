use crate::state::State;
use std::rc::Rc;
use std::cell::RefCell;


pub fn get_state_instance<'a>(state_name: &str, all_states: &Vec<Rc<RefCell<State<'a>>>>) -> Rc<RefCell<State<'a>>> {
    all_states.iter()
        .find(|s| s.borrow().name == state_name)
        .unwrap()
        .clone()
}

pub fn get_transition_states<'a>(transition_states: &Vec<&str>, all_states: &Vec<Rc<RefCell<State<'a>>>>) -> Vec<Rc<RefCell<State<'a>>>> {
    let mut list_of_states = Vec::new();
    for state_name in transition_states {
        // ignore # transitions a.k.a. null transitions
        if *state_name == "#" {
            continue;
        }
        list_of_states.push(get_state_instance(state_name, all_states));
    }
    list_of_states
}