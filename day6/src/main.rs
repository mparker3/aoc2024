use std::collections::HashSet;

fn main() {
    let contents = helpers::get_input("input.txt").unwrap();
    let raw_board: Vec<Vec<char>> = contents.split("\n").map(|x| x.chars().collect::<Vec<char>>()).collect();
    let mut board_state = BoardState::from(raw_board);
    let mut done = false;
    while !done {
        done = board_state.step();
    }
    let set: HashSet<_> = board_state.guard_visited.into_iter().collect();
    println!("visited: {:?}", set.len());
}
#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    // Returns (row_change, col_change)
    fn get_movement(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0), 
            Direction::East => (0, 1),  
            Direction::West => (0, -1), 
        }
    }
}

struct BoardState {
    board: Vec<Vec<char>>,
    guard_position: (i32, i32),
    guard_direction: Direction,
    guard_visited: Vec<(i32, i32)>,
    obstacle_locations: Vec<(i32, i32)>
}

impl BoardState {
    fn step(&mut self) -> bool {
        // get possible next step
        let possible_next_mvmt = self.guard_direction.get_movement();
        let possible_next = (possible_next_mvmt.0 + self.guard_position.0, possible_next_mvmt.1 + self.guard_position.1);
        // check for oob 
        if possible_next.0 < 0 || possible_next.1 < 0 || possible_next.0 == self.board.len() as i32 || possible_next.1 == self.board[0].len() as i32 {
            return true
        }

        if self.obstacle_locations.contains(&possible_next) {
            self.guard_direction = self.guard_direction.turn_right();
        }
        self.guard_position = (self.guard_position.0 + self.guard_direction.get_movement().0, self.guard_position.1 + self.guard_direction.get_movement().1);
        self.guard_visited.push(self.guard_position);
        return false
    }
}



impl From<Vec<Vec<char>>> for BoardState {
    fn from(board_data: Vec<Vec<char>>) -> Self {  // Note: fixed parameter name typo and added return type
        let mut guard_position: (i32, i32) = (-1, -1);
        let mut obstacle_locations: Vec<(i32, i32)> = Vec::new();
        for i in 0..board_data.len() {
            for j in 0..board_data[i].len() {
                if board_data[i][j] == '^' {
                    guard_position = (i as i32, j as i32);
                } else if board_data[i][j] == '#' {
                    obstacle_locations.push((i as i32, j as i32));
                }
            }
        }
        BoardState {
            board: board_data,
            guard_position: guard_position,
            guard_direction: Direction::North,
            guard_visited: vec![guard_position],
            obstacle_locations: obstacle_locations
        }
    }
}
