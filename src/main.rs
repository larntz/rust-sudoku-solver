use anyhow::Result;
use std::io::prelude::*;
use std::time::Instant;

fn main() -> Result<()> {
    print!("Let's Sudoku!\nEnter puzzle: ");
    let mut puzzle = String::new();
    std::io::stdout().flush().unwrap();
    let _ = std::io::stdin().read_line(&mut puzzle).unwrap();
    let mut sudoku_board = [0u8; 81];
    let mut index = 0;
    for digit in puzzle.trim().chars().into_iter() {
        sudoku_board[index] = digit.to_digit(10).unwrap() as u8;
        index += 1;
    }

    let start_time = Instant::now();
    let answer = solve(sudoku_board);
    let duration = start_time.elapsed();

    println!("\nCalculated solution in {}ms\n\n", duration.as_millis());
    print_board(&answer.1);

    Ok(())
}

fn solve(mut sudoku_board: [u8; 81]) -> (bool, [u8; 81]) {
    if let Some(zero_index) = sudoku_board.iter().position(|&x| x == 0) {
        for digit in 1..=9 {
            if check_column(sudoku_board, zero_index, digit)
                && check_row(sudoku_board, zero_index, digit)
                && check_box(sudoku_board, zero_index, digit)
            {
                sudoku_board[zero_index] = digit;
                let answer = solve(sudoku_board);
                if answer.0 {
                    return answer;
                }
            } else {
                continue;
            }
        }
    } else {
        return (true, sudoku_board);
    }
    (false, sudoku_board)
}

fn check_box(sudoku_board: [u8; 81], index: usize, digit: u8) -> bool {
    let box_number = ((index / 27) * 3) + ((index % 9) / 3);
    let mut matches = 0;
    for brow in 0..3 {
        for bcol in 0..3 {
            let box_index = (box_number / 3) * 27 + bcol + (brow * 9) + (box_number * 3)
                - ((box_number / 3) * 9);
            if sudoku_board[box_index] == digit {
                matches += 1;
            }
        }
    }
    if matches > 0 {
        return false;
    }
    true
}

fn check_column(sudoku_board: [u8; 81], index: usize, digit: u8) -> bool {
    let mut column_index = index % 9;
    let mut matches = 0;
    while column_index < sudoku_board.len() {
        if digit == sudoku_board[column_index] {
            matches += 1;
        }
        column_index += 9;
    }

    if matches > 0 {
        return false;
    }

    true
}

fn check_row(sudoku_board: [u8; 81], index: usize, digit: u8) -> bool {
    let row_start = index - ((index) % 9);
    let mut matches = 0;
    for x in row_start..row_start + 9 {
        if digit == sudoku_board[x] {
            matches += 1;
        }
    }

    if matches > 0 {
        return false;
    }

    true
}

fn print_board(sudoku_board: &[u8; 81]) {
    for index in 0..81 {
        print!("{} ", sudoku_board[index]);
        if index != 0 && (index + 1) % 3 == 0 && (index + 1) % 9 != 0 {
            print!("| ");
        }
        if index != 0 && (index + 1) % 9 == 0 {
            print!("\n");
        }
        if index + 1 == 27 || index + 1 == 54 {
            print!("---------------------\n");
        }
    }

    std::io::stdout().flush().unwrap();
    println!("");
}
