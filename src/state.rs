pub struct State{
    matrix :  [[i32; 3]; 3],
    current_player : i8, 
}

impl State{
    pub fn new() -> Self {
        Self( [ [0, 0, 0], [0, 0, 0], [0, 0, 0] ], 1);
    }

    pub fn is_final_state(&self) -> (bool, i8){
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
                return (true, self.matrix[i][j]);
            }

            
            if self.matrix[0][i] != 0 && self.matrix[0][i] == self.matrix[1][i] && self.matrix[2][i] == self.matrix[1][i]{
                return (true, self.matrix[i][j]);
            }
        }
    }
}