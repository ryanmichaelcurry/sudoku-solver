/*
Author: Ryan Michael Curry
Sources:
https://www.thonky.com/sudoku/evaluate-sudoku
https://www.yagiz.co/sudoku-generating-valid-one
*/
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::thread;
use std::time::Duration;
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

//print!("\x1B[2J\x1B[H"); // ChatGPT :)
fn sudoku_string(board: &Vec<Vec<usize>>) -> String {
    let mut output = String::new();
    for row in board {
        for value in row {
            if (*value as i32) == 0 {
                output.push_str(".");
            } else {
                output.push_str(&format!("{}", value));
            }
        }
    }
    return output;
}
fn print_sudoku_board(board: &Vec<Vec<usize>>, size: usize, melatonin: i32) {
    let sqrt_size = (size as f32).sqrt() as usize;
    let mut output = String::new();

    if melatonin > -1 {
        print!("\x1B[2J\x1B[H"); // ChatGPT :)
    }

    for (i, row) in board.iter().enumerate() {
        if i % sqrt_size == 0 && i != 0 {
            output.push_str(" "); // start --- one space right
            // Append a horizontal separator for each block
            if size > 9 {
                output.push_str(&"-".repeat(size * 4 + sqrt_size - 1)); // we are appending an extra space for two-digit sudoku boards (space + digit + space or digit + space == 4)
            } else {
                output.push_str(&"-".repeat(size * 3 + sqrt_size - 1)); // space + digit + space == 3
            }
            output.push('\n');
        }
        for (j, cell) in row.iter().enumerate() {
            if j % sqrt_size == 0 && j != 0 {
                // Append a vertical separator for each block
                output.push_str(" | ");
            } else {
                output.push_str("  ");
            }

            if size > 9 {
                if *cell >= 10 {
                    output.push_str(&format!("{}", cell));
                } else {
                    output.push_str(&format!("{} ", cell));
                }
            } else {
                output.push_str(&format!("{}", cell));
            }
        }
        output.push('\n');
    }
    println!("{}", output);
}

fn is_valid(board: &Vec<Vec<usize>>, size: usize, x: usize, y: usize, value: usize) -> bool {
    // Check row and column
    for i in 0..size {
        if board[x][i] == value || board[i][y] == value {
            return false;
        }
    }

    // Check inner square
    let sqrt_size = (size as f64).sqrt() as usize;
    let x_start = (x / sqrt_size) * sqrt_size; // inner square x (for 9x9 its either 0 1 2)
    let y_start = (y / sqrt_size) * sqrt_size; // inner square y (for 9x9 its either 0 1 2)
    for i in x_start..x_start + sqrt_size {
        for j in y_start..y_start + sqrt_size {
            if board[i][j] == value {
                return false;
            }
        }
    }

    return true;
}

fn find_available_positions(
    board: &Vec<Vec<usize>>,
    size: usize,
    x: usize,
    y: usize
) -> Vec<usize> {
    let mut available_positions = Vec::new();
    for value in 1..=size as usize {
        if is_valid(board, size, x, y, value) {
            available_positions.push(value as usize);
        }
    }
    return available_positions;
}

fn get_next(board: &Vec<Vec<usize>>, size: usize) -> (usize, usize, Vec<usize>) {
    let mut smallest_space = usize::MAX;
    let mut best_x: usize = 0;
    let mut best_y: usize = 0;
    let mut best_available_positions = Vec::new();

    for x in 0..size {
        for y in 0..size {
            if board[x][y] == 0 {
                let available_positions = find_available_positions(board, size, x, y);
                if available_positions.len() < smallest_space {
                    smallest_space = available_positions.len();
                    best_x = x;
                    best_y = y;
                    best_available_positions = available_positions;
                }
            }
        }
    }

    (best_x, best_y, best_available_positions)
}

/*
fn sudoku_solver(
    board: &mut Vec<Vec<usize>>,
    size: usize,
    melatonin: i32
) -> bool {
    // check to see if there are blank spaces
    let mut is_full = true;
    for row in &*board {
        for &value in row {
            if value == 0 {
                is_full = false;
                break;
            }
        }
    }

    if is_full {
        return true;  // we are done
    }

    // Use get_next to find the cell with the least number of possible values
    let (x, y, mut available_positions) = get_next(board, size);
    available_positions.shuffle(&mut thread_rng());

    // Melatonin Factor (show each step)
    if melatonin > -1 {
        if melatonin > 0 {
            let sleep_duration = Duration::from_millis(melatonin as u64);
            thread::sleep(sleep_duration);
        }
        print_sudoku_board(board, size, melatonin);
    }

    // Attempt to fill the chosen cell with one of the possible values
    for &value in &available_positions {
        if is_valid(board, size, x, y, value) {
            board[x][y] = value;
            if sudoku_solver(board, size, melatonin) {
                return true;
            }
            board[x][y] = 0;  // Reset cell and backtrack
        }
    }

    return false;
}
*/


fn sudoku_solver(
    board: &mut Vec<Vec<usize>>,
    size: usize,
    x: usize,
    y: usize,
    melatonin: i32
) -> bool {
    if x == size {
        return true;
    }

    if board[x][y] != 0 {
        if y == size - 1 {
            if sudoku_solver(board, size, x + 1, 0, melatonin) {
                return true;
            }
        } else {
            if sudoku_solver(board, size, x, y + 1, melatonin) {
                return true;
            }
        }
        return false;
    }

    // Melatonin Factor (show each step)
    if melatonin > -1 {
        if melatonin > 0 {
            let sleep_duration = Duration::from_millis(melatonin as u64);
            thread::sleep(sleep_duration);
        }
        print_sudoku_board(board, size, melatonin);
    }

    let mut rng = thread_rng();
    let mut available_positions = find_available_positions(board, size, x, y);

    // Shuffle available positions for randomness
    available_positions.shuffle(&mut rng);

    for &value in &available_positions {
        board[x][y] = value as usize;
        if y == size - 1 {
            if sudoku_solver(board, size, x + 1, 0, melatonin) {
                return true;
            }
        } else {
            if sudoku_solver(board, size, x, y + 1, melatonin) {
                return true;
            }
        }
    }

    // If no position is available, backtrack
    board[x][y] = 0;

    // Melatonin Factor (show each step)
    if melatonin > -1 {
        if melatonin > 0 {
            let sleep_duration = Duration::from_millis(melatonin as u64);
            thread::sleep(sleep_duration);
        }
        print_sudoku_board(board, size, melatonin);
    }

    return false;
}


fn collect_sudoku_solutions(
    board: &mut Vec<Vec<usize>>,
    size: usize,
    x: usize,
    y: usize,
    solutions: &mut usize,
    melatonin: i32
) {
    if x == size {
        *solutions += 1;
        return;
    }

    if *solutions > 1 {
        return;
    }

    // Melatonin Factor (show each step)
    if melatonin > -1 {
        if melatonin > 0 {
            let sleep_duration = Duration::from_millis(melatonin as u64);
            thread::sleep(sleep_duration);
        }
        print_sudoku_board(board, size, melatonin);
    }

    if board[x][y] != 0 {
        if y == size - 1 {
            collect_sudoku_solutions(board, size, x + 1, 0, solutions, melatonin);
        } else {
            collect_sudoku_solutions(board, size, x, y + 1, solutions, melatonin);
        }
        return;
    }

    let mut rng = thread_rng();
    let mut available_positions = find_available_positions(board, size, x, y);

    // Shuffle available positions for randomness
    available_positions.shuffle(&mut rng);

    for &value in &available_positions {
        board[x][y] = value as usize;
        if y == size - 1 {
            collect_sudoku_solutions(board, size, x + 1, 0, solutions, melatonin);
        } else {
            collect_sudoku_solutions(board, size, x, y + 1, solutions, melatonin);
        }
    }

    // If no position is available, backtrack
    board[x][y] = 0;

    // Melatonin Factor (show each step)
    if melatonin > -1 {
        if melatonin > 0 {
            let sleep_duration = Duration::from_millis(melatonin as u64);
            thread::sleep(sleep_duration);
        }
        print_sudoku_board(board, size, melatonin);
    }
}

fn sudoku_unique(board: &mut Vec<Vec<usize>>, size: usize, melatonin: i32) -> bool {
    let mut solutions = 0;
    collect_sudoku_solutions(board, size, 0, 0, &mut solutions, melatonin);
    // println!("{}", solutions);
    return solutions == 1;
}

fn generate_unique(board: &mut Vec<Vec<usize>>, size: usize, difficulty: usize, melatonin: i32) {
    let mut eraser: i32 = (difficulty * size) as i32;
    if eraser > ((size * size) as i32) {
        eraser = (size * size) as i32;
    }

    let mut not_visited: Vec<Vec<usize>> = vec![];
    for x in 0..size {
        for y in 0..size {
            not_visited.push(vec![x, y]);
        }
    }
    /*
    for cell in &not_visited {
        for pos in cell {
            print!("{} ", pos);
        }
        println!();
    }
    */
    not_visited.shuffle(&mut thread_rng());

    while eraser > 0 {
        // println!("{}", eraser);
        if not_visited.len() > 0 {
            let temp = board[not_visited[0][0]][not_visited[0][1]];
            board[not_visited[0][0]][not_visited[0][1]] = 0;
            if !sudoku_unique(board, size, melatonin) {
                board[not_visited[0][0]][not_visited[0][1]] = temp;
                not_visited.drain(0..1);
            } else {
                not_visited.drain(0..1);
                eraser -= 1;
            }
        } else {
            return;
        }
    }
}

fn anakin(board: &mut Vec<Vec<usize>>, size: usize, difficulty: usize) {
    let mut eraser: i32 = (difficulty * size) as i32;
    if eraser > ((size * size) as i32) {
        eraser = (size * size) as i32;
    }

    let mut not_visited: Vec<Vec<usize>> = vec![];
    for x in 0..size {
        for y in 0..size {
            not_visited.push(vec![x, y]);
        }
    }

    /*
    for cell in &not_visited {
        for pos in cell {
            print!("{} ", pos);
        }
        println!();
    }
    */
    not_visited.shuffle(&mut thread_rng());
    not_visited.drain(0..((size * size) - eraser as usize));

    for position in not_visited {
        board[position[0]][position[1]] = 0;
    }
}

fn main() {
    let mut results = String::new();

    let args: Vec<_> = env::args().collect(); // StackOverflow (various posts to turn cli input into a variable)

    let size = if args.len() > 1 {
        match args[1].parse() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Invalid size argument. Using default size: 9");
                9
            }
        }
    } else {
        9
    };

    let difficulty = if args.len() > 2 {
        match args[2].parse() {
            Ok(m) => m,
            Err(_) => {
                eprintln!("Invalid difficulty argument. Using default difficulty: 5");
                2
            }
        }
    } else {
        2
    };

    let melatonin = if args.len() > 3 {
        match args[3].parse() {
            Ok(m) => m,
            Err(_) => {
                eprintln!("Invalid melatonin argument. Using default melatonin: -1");
                -1
            }
        }
    } else {
        -1
    };

    for _ in 1..=100 {
        let mut board = vec![vec![0; size]; size];
        let mut board_string: String;

        let generator_start_time = Instant::now();
        sudoku_solver(&mut board, size, 0, 0, melatonin);
        let generator_elapsed_time = generator_start_time.elapsed().as_micros();

        print_sudoku_board(&board, size, melatonin);

        let unique_start_time = Instant::now();
        if size <= 9 {
            generate_unique(&mut board, size, difficulty, melatonin);
        }
        else {
            anakin(&mut board, size, difficulty);
        }
        let unique_elapsed_time = unique_start_time.elapsed().as_micros();
        
        board_string = sudoku_string(&board);
        print_sudoku_board(&board, size, melatonin);
        //sudoku_string(&board);

        let solver_start_time = Instant::now();
        sudoku_solver(&mut board, size, 0, 0, melatonin);
        let solver_elapsed_time = solver_start_time.elapsed().as_micros();
        print_sudoku_board(&board, size, melatonin);

        // Append the results to the respective strings
        //results.push_str(&format!("CLI: {}, {}, {}, Generator Time: {} microseconds, Unique Time: {} microseconds, Solver Time: {} microseconds\n", size, difficulty, melatonin, generator_elapsed_time, unique_elapsed_time, solver_elapsed_time));
        //results.push_str(&format!("Board: {} \n\n", board_string));
        results.push_str(&format!("{}\n", solver_elapsed_time));
    }

    // Save merge sort results to a file
    let mut stats_file = File::create("sudoku_times_4_4_old.txt").expect("Unable to create merge sort file");
    write!(stats_file, "{}", results).expect("Unable to write to merge sort file");
}
