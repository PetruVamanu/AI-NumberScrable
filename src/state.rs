use crate::constants::{INDEXES, NUMBER_SQUARE};
#[derive(Clone)]
pub struct State{
    matrix :  [[i32; 3]; 3],
    pub current_player : i32, 
}

impl State{
    pub fn new() -> Self {
        Self {  matrix: [ [0, 0, 0], [0, 0, 0], [0, 0, 0] ], current_player: 1 }
    }

    pub fn is_final_state(&self) -> (bool, i32){
        let mut zeros = 0;
        for i in 0..3{
            for j in 0..3{
                if self.matrix[i][j] == 0{
                    zeros += 1;
                }
            }
        }
        
        for i in 0..3{
            if self.matrix[i][0] != 0 && self.matrix[i][0] == self.matrix[i][1] && self.matrix[i][2] == self.matrix[i][1]{
                return (true, self.matrix[i][0]);
            }

            
            if self.matrix[0][i] != 0 && self.matrix[0][i] == self.matrix[1][i] && self.matrix[2][i] == self.matrix[1][i]{
                return (true, self.matrix[0][i]);
            }
        }

        if self.matrix[0][0] != 0 && self.matrix[0][0] == self.matrix[1][1] && self.matrix[0][0] == self.matrix[2][2]{
            return (true, self.matrix[0][0]);
        }

        if self.matrix[0][2] != 0 && self.matrix[0][2] == self.matrix[1][1] && self.matrix[0][2] == self.matrix[2][0]{
            return (true, self.matrix[0][2]);
        }

        if zeros == 0{
            return (true, 0);
        }

        (false, 0)
    }
  
    pub fn is_valid_move(&self, x : i32, y : i32) -> bool{
        if x < 0 || x > 2 || y < 0 || y > 2{
            return false;
        }

        if self.matrix[x as usize][y as usize] != 0{
            return false;
        }

        return true;
    }

    pub fn make_move(&mut self, x : i32, y : i32){
        self.matrix[x as usize][y as usize] = self.current_player;
        self.current_player = 3-self.current_player;
    }

    pub fn put_number(&self, x : i32) ->Option<State>{
        let (i, j) = INDEXES[x as usize];
        let mut other = self.clone();
        if !other.is_valid_move(i,j){
            return None
        }
        other.make_move(i, j);
        Some(other)
    }

    pub fn print(&self){
        for i in 0..3{
            
            for j in 0..3{
                print!("{} ", NUMBER_SQUARE[i][j]);
            }
            print!("        ");
            for j in 0..3{
                print!("{} ", self.matrix[i][j]);
            }
            println!();
        }
    }

    pub fn heuristic(&self, player: i32) -> i32{
        let mut score = 0 as i32;
        for i in 0..3{
            if self.matrix[i][0] != player &&
                self.matrix[i][1] != player &&
                self.matrix[i][2] != player{
                     score += 1;
                }

            if self.matrix[0][i] != player &&
                self.matrix[1][i] != player &&
                self.matrix[2][i] != player{
                     score += 1;
                }
        }

        if self.matrix[0][0] != player &&
            self.matrix[1][1] != player &&
            self.matrix[2][2] != player{
                 score += 1;
            }

        if self.matrix[0][2] != player &&
            self.matrix[1][1] != player &&
            self.matrix[2][0] != player{
                 score += 1;
            }
        score
    }

    pub fn min_max_heuristic(&self) -> i32{
        let (is_final, winner) = self.is_final_state();
        if is_final{
            if winner == 1{
                return 100;
            }
            else if winner == 2 {return -100;}
            else {
                return  self.heuristic(3-self.current_player) - self.heuristic(self.current_player);
            }
        }
        self.heuristic(3-self.current_player) - self.heuristic(self.current_player)
    }


    pub fn get_next_move(&self) -> Option<State>{
        for i in 1..10{
            let result = self.put_number(i);
            if result.is_some(){
                let new_state = result.unwrap();
                return Some(new_state);
            }
        }
        None
    }
}