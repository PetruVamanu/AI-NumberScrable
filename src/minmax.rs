use crate::state::State;

pub fn minmax(state: &State, depth: i32, is_max_player: bool) -> i32 {
    if depth == 0 || state.is_final_state().0 {
        return state.min_max_heuristic();
    }
    let mut value = 0;
    if is_max_player {
        value = -1000;
        for i in 1..10 {
            let new_state = state.put_number(i);

            match new_state {
                Some(my_new_state) => {
                    let new_value = minmax(&my_new_state, depth - 1, false);
                    if new_value > value {
                        value = new_value;
                    }
                }
                None => {},
            }
        }
    } else {
        value = 1000;
        for i in 1..10 {
            let new_state = state.put_number(i);
            match new_state {
                Some(my_new_state) => {
                    let new_value = minmax(&my_new_state, depth - 1, true);
                    if new_value < value {
                        value = new_value;
                    }
                }
                None => {},
            }
        }
    }
    value
}

pub fn get_next_move(state: &State) -> Option<State> {
    let mut value = i32::MAX;
    let mut next_state = None;
    for i in 1..10 {
        let new_state_option = state.put_number(i);
        match new_state_option {
            Some(new_state) => {
                let new_value = minmax(&new_state, 5, true);
                if new_value < value {
                    value = new_value;
                    next_state = Some(new_state.clone());
                }
            },
            None => {},
        }
       
    }
    next_state
}
