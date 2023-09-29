use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_sudoku_board(board_size: usize, square_count: usize) -> Vec<Vec<usize>> {
    let mut board = vec![vec![0; board_size]; board_size];
    let mut forbidden_positions: Vec<Vec<usize>> = vec![vec![]; board_size];

    for x in 1..=board_size {
        let mut y = 0;
        let mut previous_positions: Vec<usize> = vec![];

        while y < square_count {
            let mut available_positions: Vec<usize> = (0..board_size).collect();

            // Filter positions based on forbidden positions
            available_positions.retain(|&pos| !forbidden_positions[x - 1].contains(&pos));

            // Filter positions based on previous positions in the same x
            available_positions.retain(|&pos| !previous_positions.contains(&pos));

            if !available_positions.is_empty() {
                // Select a random position from the filtered list
                let mut rng = thread_rng();
                let chosen_position = available_positions.choose(&mut rng).unwrap();

                // Update the board and previous positions
                board[x - 1][*chosen_position] = x;
                previous_positions.push(*chosen_position);
                y += 1;
            } else {
                // If no positions are available, backtrack
                if let Some(last_position) = previous_positions.pop() {
                    forbidden_positions[x - 1].push(last_position);
                }
                y -= 1;
            }
        }
    }

    board
}

fn main() {
    let board_size = 9; // 9x9 for a standard Sudoku
    let square_count = 3; // 3 for a standard Sudoku
    
    let sudoku_board = generate_sudoku_board(board_size, square_count);

    // Print the generated Sudoku board
    for row in sudoku_board.iter() {
        println!("{:?}", row);
    }
}
