use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::env;

//print!("\x1B[2J\x1B[H"); // ChatGPT :)

fn print_sudoku_board(board: &Vec<Vec<usize>>, size: usize) {
    let sqrt_size = (size as f32).sqrt() as usize;
    let mut output = String::new();

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
    for i in x_start..sqrt_size {
        for j in y_start..sqrt_size {
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
        print_sudoku_board(board, size);
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
        print_sudoku_board(board, size);
    }

    return false;
}

fn anakin(board: &mut Vec<Vec<usize>>, size: usize, difficulty: usize) {
    let mut eraser: i32 = (difficulty * size) as i32;
    if eraser > (size * size) as i32 {
        eraser = (size * size) as i32;
    }

    while eraser > 0 {
        let mut x = rand::thread_rng().gen_range(0..size);
        let mut y = rand::thread_rng().gen_range(0..size);
        if board[x][y] != 0 {
            board[x][y] = 0;
            eraser -= 1;
        }
    }
}

fn main() {
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
                eprintln!("Invalid difficulty argument. Using default difficulty: 2");
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

    let mut board = vec![vec![0; size]; size];

    sudoku_solver(&mut board, size, 0, 0, melatonin);
    print_sudoku_board(&board, size);

    anakin(&mut board, size, difficulty);
    print_sudoku_board(&board, size);

    sudoku_solver(&mut board, size, 0, 0, melatonin);
    print_sudoku_board(&board, size);
}
