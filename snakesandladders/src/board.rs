pub struct Board {
    snakes: Vec<(i32, i32)>,
    ladders: Vec<(i32, i32)>
}


pub fn get_board () -> Board {
    let board = Board{snakes : vec ![(23, 12)], ladders : vec ![(2, 9), (3, 6), (4, 12), (25, 34)]};

    board

}

pub fn get_new_position (board : &Board, roll_pos : i32) -> i32 {
    for &(first, second) in &board.snakes {
       if first == roll_pos {
           return second
       }
    }

    for &(first, second) in &board.ladders {
        if first == roll_pos {
            return second
        }
    }

    return roll_pos

}
