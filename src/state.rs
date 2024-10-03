use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub struct State<'a> {
    pub name: &'a str,
    pub transitions: HashMap<&'a str, Vec<Rc<RefCell<State<'a>>>>>,
    pub epsilon_transitions: HashMap<&'a str, Vec<Rc<RefCell<State<'a>>>>>
}

impl<'a> State<'a> {
    pub fn new(name: &'a str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(State {
            name,
            transitions: HashMap::new(),
            epsilon_transitions: HashMap::new(),
        }))
    }

    pub fn check_for_epsilon_transitions(&self) -> Vec<Rc<RefCell<State<'a>>>> {
        let mut epsilon_states = Vec::new();
        
        if let Some(states) = self.epsilon_transitions.get("$") {
            for state in states {
                epsilon_states.push(state.clone());
                // Recursively check
                let next_states = state.borrow().check_for_epsilon_transitions();
                epsilon_states.extend(next_states);
            }
        }

        epsilon_states
    }

    pub fn check_for_symbol_transition(&self, symbol: &str) -> Vec<Rc<RefCell<State<'a>>>> {
        let mut transition_states = Vec::new();
        if let Some(states) = self.transitions.get(symbol) {
            for state in states {
                transition_states.push(state.clone());
            }
        }
        transition_states
    }
}

impl<'a> fmt::Display for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}