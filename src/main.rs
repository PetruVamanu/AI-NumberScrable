use std::process::exit;

use tic_tac_toe::{state::State, minmax::get_next_move};

fn main() {
    let mut actual_state = State::new();
    println!("{}", actual_state.is_final_state().0);
    while !actual_state.is_final_state().0 {
        actual_state.print();
        if actual_state.current_player == 1 {
            println!("Your turn");

            println!("Enter the number");
            let mut x = String::new();
            std::io::stdin()
                .read_line(&mut x)
                .expect("Failed to read number");

            let new_state = actual_state.put_number(x.trim().parse().unwrap());
            match new_state {
                Some(state) => actual_state = state,
                None => println!("Invalid move"),
            }
            
        } else {
            match get_next_move(&actual_state){
                Some(state) => actual_state = state.clone(),
                None => {println!("No more moves");
                exit(0);},
            };
        }
    }
    match actual_state.is_final_state() {
        (true, 1) => println!("You win"),
        (true, 2) => println!("You lose"),
        (true, 0) => println!("Draw"),
        _ => println!("Error"),
    }
}
