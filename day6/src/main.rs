use std::{collections::HashSet, thread, time::Instant};

fn main() {
    let now = Instant::now();
    let contents = helpers::get_input("input.txt").unwrap();
    let raw_board: Vec<Vec<char>> = contents.split("\n").map(|x| x.chars().collect::<Vec<char>>()).collect();
    let board_height = raw_board.len();
    let board_width = raw_board[0].len();
    let board_state = BoardState::from(raw_board);
    let mut incompletable = 0;

    let num_threads = thread::available_parallelism().unwrap().get();
    println!("running with {} threads", num_threads);
    let chunk_size = (board_height + num_threads - 1) / num_threads;

    let mut handles = vec![]; // this is ~similar to a go waitgroup. we'll be collecting references to our threads and `joining` for each of them.

    let board_for_spawn = board_state.clone();
    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = ((i+1) * chunk_size).min(board_height);
        let board_for_thread = board_for_spawn.clone();
        let handle = thread::spawn(move || {
            let mut incompletable = 0;
            for i in start..end{
                for j in 0..board_width {
                    let mut new_board_state = board_for_thread.clone();
                    if let Ok(_) = new_board_state.insert_obstacle((i as i32, j as i32)) { // o(obstacles)
                        println!("{} {}", i, j);
                        if !new_board_state.check_completable() {
                            incompletable += 1;
                        }
                    }
                }
            }
            incompletable
        });
        handles.push(handle)
    }
    let mut overall_incompletable = 0;
    for h in handles {
        overall_incompletable += h.join().unwrap();
    }
    println!("incompletable: {:?}", overall_incompletable);
    println!("elapsed: {:?}", now.elapsed())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq)]
struct BoardState {
    board: Vec<Vec<char>>,
    guard_position: (i32, i32),
    guard_direction: Direction,
    guard_visited: HashSet<((i32, i32), Direction)>,
    obstacle_locations: HashSet<(i32, i32)>
}

impl BoardState {
    fn step(&mut self) -> (bool, bool) {
        let completable = true;
        let incompletable = false;
        // get possible next step
        let possible_next_mvmt = self.guard_direction.get_movement();
        let possible_next = (possible_next_mvmt.0 + self.guard_position.0, possible_next_mvmt.1 + self.guard_position.1);
        // check for oob 
        if possible_next.0 < 0 || possible_next.1 < 0 || possible_next.0 == self.board.len() as i32 || possible_next.1 == self.board[0].len() as i32 {
            return (true, completable)
        }

        if self.obstacle_locations.contains(&possible_next) { //O(1)
            self.guard_direction = self.guard_direction.turn_right();
            return (false, completable)
        }
        self.guard_position = (self.guard_position.0 + self.guard_direction.get_movement().0, self.guard_position.1 + self.guard_direction.get_movement().1);
        if self.guard_visited.contains(&(self.guard_position, self.guard_direction)) { // O(1))
            return (false, incompletable)
        }
        self.guard_visited.insert((self.guard_position, self.guard_direction));
        return (false, completable)
    }

    fn insert_obstacle(&mut self, coords: (i32, i32)) -> Result<(), ()> {
        match coords {
            gp if gp == self.guard_position => Err(()), 
            o if self.obstacle_locations.contains(&o) => Err(()),
            _ => {
                self.obstacle_locations.insert(coords); 
                self.board[coords.0 as usize][coords.1 as usize] = '#';
                Ok(())
            }
        }
    }

    fn print_board_state(&mut self) -> String {
        return self.board.clone().into_iter().map(|x| x.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")
    }

    fn check_completable(&mut self) -> bool {
        let mut done = false;
        let mut completeable = true;
        while !done && completeable {
            (done, completeable) = self.step();
        } 
        return completeable
    }
}



impl From<Vec<Vec<char>>> for BoardState {
    fn from(board_data: Vec<Vec<char>>) -> Self {  // Note: fixed parameter name typo and added return type
        let mut guard_position: (i32, i32) = (-1, -1);
        let mut obstacle_locations: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..board_data.len() {
            for j in 0..board_data[i].len() {
                if board_data[i][j] == '^' {
                    guard_position = (i as i32, j as i32);
                } else if board_data[i][j] == '#' {
                    obstacle_locations.insert((i as i32, j as i32));
                }
            }
        }
        BoardState {
            board: board_data,
            guard_position: guard_position,
            guard_direction: Direction::North,
            guard_visited: HashSet::from_iter(vec![(guard_position, Direction::North)]),
            obstacle_locations: obstacle_locations
        }
    }
}
