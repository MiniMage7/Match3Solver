use serde::Deserialize;
use serde_json;
use std::io::{self, Write};
use std::thread;
use std::sync::mpsc;

#[derive(Deserialize, Debug)]
struct GameBoard {
    height: usize,
    width: usize,
    board: Vec<Vec<isize>>,
}

#[derive(Clone)]
struct Swap {
    y1: usize,
    x1: usize,
    y2: usize,
    x2: usize,
}


fn main() {
    // Get the game board as a JSON formatted string from the user
    let mut game_board = String::new();

    println!("Paste your exported board here:");
    println!("(You can get your board by drawing it on the website and clicking the export button)");
    print!("> ");

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut game_board)
        .expect("Failed to read line.");

    // Format the input into a GameBoard struct
    let game_board: GameBoard = serde_json::from_str(&game_board).unwrap_or_else(|_error| panic!("That is not a valid gameboard."));

    // Get the number of thread layers from the user
    let mut max_thread_depth = String::new();

    println!("How mant layers of threads do you want to spawn?");
    println!("2 is recommended for most solves. 1 might be neccessary for the larger solves. Larger numbers are faster but more intensive.");
    print!("> ");
    
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut max_thread_depth)
        .expect("Failed to read line.");

    let max_thread_depth = max_thread_depth.parse::<usize>().unwrap_or_else(|_error| panic!("That is not a valid number."));

    let mut moves_to_solve : Vec<Swap> = Vec::new();

    moves_to_solve = solve(game_board, moves_to_solve, max_thread_depth);

    println!();
    if moves_to_solve.is_empty() {
        println!("The puzzle is impossible.")
    }
    for swap in moves_to_solve {
        println!("Swap {}, {} with {}, {}", swap.y1, swap.x1, swap.y2, swap.x2);
    }
}


// Checks if the board has any valid moves and spawns a new thread to do any found moves
// Those threads will do all the logic for the move and then start from this function again
fn solve(mut game_board: GameBoard, moves_to_solve : Vec<Swap>, thread_depth_remaining : usize) -> Vec<Swap> {
    if check_for_loss(&game_board) {return Vec::new()}; //  Return an empty vector to show failure

    // Communication between threads
    let (tx, rx) = mpsc::channel();

    // For each row in the grid
    for y in 0..game_board.height {
        // For each column in the grid
        for x in 0..game_board.width {
            // If the tile is movable
            if game_board.board[y][x] > 0 {
                // Check if the piece can be moved down or right
                // There is no need to check up or left as those would be
                // another piece's down or right respectively

                // Swap Down
                if check_if_valid_move(&mut game_board, Swap{y1:y, x1:x, y2:y + 1, x2:x}) {
                    let game_board_copy = GameBoard{
                        board: game_board.board.clone(),
                        ..game_board
                    };
                    let moves_to_solve_copy = moves_to_solve.to_vec();
                    let tx1 = tx.clone();

                    // If there is thread depth remaining, make a new thread
                    if thread_depth_remaining > 0 {
                        // Spawn a new thread to execute the move and continue the process
                        thread::spawn(move || {
                            let moves_to_solve_new =
                                execute_move(game_board_copy, Swap{y1:y, x1:x, y2:y + 1, x2:x}, moves_to_solve_copy, thread_depth_remaining - 1);
                            // If moves to solve is empty here, it failed, so only send if not empty
                            if !moves_to_solve_new.is_empty(){
                                tx1.send(moves_to_solve_new).unwrap_or_else(|_error| return);
                            }
                        });
                    }
                    // Otherwise run the code in this thread
                    else {
                        let moves_to_solve_new =
                                execute_move(game_board_copy, Swap{y1:y, x1:x, y2:y + 1, x2:x}, moves_to_solve_copy, 0);
                        if !moves_to_solve_new.is_empty() {
                            return moves_to_solve_new;
                        }
                    }
                }
                // Swap Right
                if check_if_valid_move(&mut game_board, Swap{y1:y, x1:x, y2:y, x2:x + 1}) {
                    let game_board_copy = GameBoard{
                        board: game_board.board.clone(),
                        ..game_board
                    };
                    let moves_to_solve_copy = moves_to_solve.to_vec();
                    let tx1 = tx.clone();

                    // If there is thread depth remaining, make a new thread
                    if thread_depth_remaining > 0 {
                        // Spawn a new thread to execute the move and continue the process
                        thread::spawn(move || {
                            let moves_to_solve_new =
                                execute_move(game_board_copy, Swap{y1:y, x1:x, y2:y, x2:x + 1}, moves_to_solve_copy, thread_depth_remaining - 1);
                            // If moves to solve is empty here, it failed, so only send if not empty
                            if !moves_to_solve_new.is_empty(){
                                tx1.send(moves_to_solve_new).unwrap_or_else(|_error| return);
                            }
                        });
                    }
                    // Otherwise run the code in this thread
                    else {
                        let moves_to_solve_new =
                                execute_move(game_board_copy, Swap{y1:y, x1:x, y2:y, x2:x + 1}, moves_to_solve_copy, 0);
                        if !moves_to_solve_new.is_empty() {
                            return moves_to_solve_new;
                        }
                    }
                }
            }
        }
    }

    // Delete the original tx so that things will panic properly if all threads finish
    drop(tx);

    // Get the moves to solve from the passed threads
    // Any threads that don't find the solution will just die
    // So this will only return something from a successful thread
    // If all the child threads fail, this will error and return an empty vector
    let moves_to_solve_result = rx.recv();
    let moves_to_solve = moves_to_solve_result.unwrap_or_else(|_error| Vec::new());

    return moves_to_solve;
}


fn check_if_valid_move(game_board: &mut GameBoard, swap: Swap) -> bool {
    // If the swap goes out of bounds
    if swap.y2 >= game_board.height || swap.x2 >= game_board.width {
        return false;
    }

    // If the move is swapping with air or a blocker
    if game_board.board[swap.y2][swap.x2] < 1 {
        return false;
    }

    // Swap the 2 spots on the puzzle board
    let temp_value = game_board.board[swap.y1][swap.x1];
    game_board.board[swap.y1][swap.x1] = game_board.board[swap.y2][swap.x2];
    game_board.board[swap.y2][swap.x2] = temp_value;

    let is_move_valid = check_if_blocks_removed(&game_board, swap.y1, swap.x1) ||
                                check_if_blocks_removed(&game_board, swap.y2, swap.x2);

    // Swap the pieces back
    let temp_value = game_board.board[swap.y1][swap.x1];
    game_board.board[swap.y1][swap.x1] = game_board.board[swap.y2][swap.x2];
    game_board.board[swap.y2][swap.x2] = temp_value;

    is_move_valid
}


// Takes an x and y coordinate of the puzzle board
// Checks if that piece should be removed
fn check_if_blocks_removed(game_board: &GameBoard, y : usize, x : usize) -> bool {
    // If it matches with the 2 blocks above it
    if y as isize - 2 >= 0 {
        if game_board.board[y - 2][x] == game_board.board[y - 1][x] &&
            game_board.board[y - 1][x] == game_board.board[y][x] {
            return true;
        }
    }
    // If it matches with 1 block above it and 1 below it
    if y as isize - 1 >= 0 && y + 1 < game_board.height {
        if game_board.board[y - 1][x] == game_board.board[y][x] &&
            game_board.board[y][x] == game_board.board[y + 1][x] {
            return true;
        }
    }
    // If it matches with the 2 blocks below it
    if y + 2 < game_board.height {
        if game_board.board[y][x] == game_board.board[y + 1][x] &&
            game_board.board[y + 1][x] == game_board.board[y + 2][x] {
            return true;
        }
    }

    // If it matches with the 2 blocks to the left of it
    if x as isize - 2 >= 0 {
        if game_board.board[y][x - 2] == game_board.board[y][x - 1] &&
            game_board.board[y][x - 1] == game_board.board[y][x] {
            return true;
        }
    }
    // If it matches with the block to the left and to the right of it
    if x as isize - 1 >= 0 && x + 1 < game_board.width {
        if game_board.board[y][x - 1] == game_board.board[y][x] &&
            game_board.board[y][x] == game_board.board[y][x + 1] {
            return true;
        }
    }
    // If it matches with the 2 blocks to the right of it
    if x + 2 < game_board.width {
        if game_board.board[y][x] == game_board.board[y][x + 1] &&
            game_board.board[y][x + 1] == game_board.board[y][x + 2] {
            return true;
        }
    }

    // If none of the moves resulted in blocks being removed
    false
}


// Executes the move and continues the solve process
fn execute_move(mut game_board: GameBoard, swap: Swap, mut moves_to_solve : Vec<Swap>, thread_depth_remaining : usize) -> Vec<Swap> {
    // Swap the 2 spots on the puzzle board
    let temp_value = game_board.board[swap.y1][swap.x1];
    game_board.board[swap.y1][swap.x1] = game_board.board[swap.y2][swap.x2];
    game_board.board[swap.y2][swap.x2] = temp_value;

    // Add the move to moves_to_solve
    moves_to_solve.push(swap);

    // Recalculate the new board as a result of that swap
    game_board = recalculate_board(game_board);

    // Check if the game is won
    if check_for_win(&game_board) {return moves_to_solve}

    // Repeat the solving process with the new board
    moves_to_solve = solve(game_board, moves_to_solve, thread_depth_remaining);

    moves_to_solve
}


// Check if there are any pieces that need to be removed and removes them
// Then rearranges the board to account for pieces falling
// Recursively calls itself until no pieces move
fn recalculate_board(mut game_board: GameBoard) -> GameBoard {
    // Get what blocks need to be removed
    let blocks_to_remove = check_what_blocks_to_remove(&game_board);

    let mut are_blocks_to_remove = false;
    // For each row in the grid
    'outer_loop: for y in 0..game_board.height {
        // For each column in the grid
        for  x in 0..game_board.width {
            // If the tile is to be removed
            if blocks_to_remove[y][x] == 1 {
                are_blocks_to_remove = true;
                break 'outer_loop;
            }
        }
    }

    // If there are any blocks to remove
    if are_blocks_to_remove {
        // Remove the blocks
        game_board = remove_given_blocks(game_board, blocks_to_remove);
        // Make all the blocks fall down
        game_board = calculate_gravity(game_board);
        // Restart this process
        game_board = recalculate_board(game_board);
    }

    game_board
}


// Iterates over the whole puzzle board and returns what blocks need to be removed
// Return: An 2d vector of 0s and 1s where 1s represent the positions where blocks need to be removed
fn check_what_blocks_to_remove(game_board: &GameBoard) -> Vec<Vec<usize>> {
    let mut blocks_to_remove : Vec<Vec<usize>> = Vec::new();

    // Fill blockToRemove with 0s
    // For each row in the grid
    for _y in 0..game_board.height {
        // For each column in the grid
        let mut column_blocks_to_remove : Vec<usize> = Vec::new();
        for _x in 0..game_board.width {
            column_blocks_to_remove.push(0);
        }
        blocks_to_remove.push(column_blocks_to_remove);
    }

    // Find the blocks that are supposed to be removed and change their marks to 1s
    // For each row in the grid
    for y in 0..game_board.height {
        // For each column in the grid
        for x in 0..game_board.width {
            // If that piece is removable
            if game_board.board[y][x] > 0 {
                // We only have to check for these 2 because all the other circumstances will be checked
                // in another piece's 2 below or 2 to the right

                // Check if it can be matched with the 2 pieces below it
                if y + 2 < game_board.height {
                    if game_board.board[y + 2][x] == game_board.board[y + 1][x] &&
                        game_board.board[y + 1][x] == game_board.board[y][x] {
                        // Mark the pieces to be removed
                        blocks_to_remove[y + 2][x] = 1;
                        blocks_to_remove[y + 1][x] = 1;
                        blocks_to_remove[y][x] = 1;
                    }
                }

                // Check if it can be matched with the 2 pieces to the right of it
                if x + 2 < game_board.width {
                    if game_board.board[y][x] == game_board.board[y][x + 1] &&
                        game_board.board[y][x + 1] == game_board.board[y][x + 2] {
                        // Mark the pieces to be removed
                        blocks_to_remove[y][x] = 1;
                        blocks_to_remove[y][x + 1] = 1;
                        blocks_to_remove[y][x + 2] = 1;
                    }
                }
            }
        }
    }

    blocks_to_remove
}

// Takes an array of 1s and 0s and the GameBoard
// Removes all blocks from the puzzle board where the given array has a 1 in the same position
// Used with check_what_blocks_to_remove()
fn remove_given_blocks(mut game_board: GameBoard, blocks_to_remove : Vec<Vec<usize>>) -> GameBoard {
    // For each row in the grid
    for y in 0..game_board.height {
        // For each column in the grid
        for  x in 0..game_board.width {
            // If the tile is to be removed
            if blocks_to_remove[y][x] == 1 {
                // Remove the tile
                game_board.board[y][x] = 0;
            }
        }
    }

    game_board
}


// Moves all blocks with air under them down 1
// Recursively calls itself until no blocks move
// Blockers (-1s) do not fall
fn calculate_gravity(mut game_board: GameBoard) -> GameBoard {
    let mut did_blocks_move = false;

    // For each row in the grid (bottom to top skipping bottom most row)
    for y in (0..game_board.height - 1).rev() {
        // For each column in the grid
        for x in 0..game_board.width {
            // If the piece is affected by gravity
            if game_board.board[y][x] > 0 {
                // If there is air below it
                if game_board.board[y + 1][x] == 0 {
                    // Move it down to that block
                    game_board.board[y + 1][x] = game_board.board[y][x];
                    game_board.board[y][x] = 0;
                    did_blocks_move = true;
                }
            }
        }
    }

    // If any blocks moved, check if more gravity is needed
    if did_blocks_move {
        game_board = calculate_gravity(game_board);
    }

    game_board
}


// Check if the puzzle is solved
fn check_for_win(game_board: &GameBoard) -> bool {
    // For each row in the grid
    for y in 0..game_board.height {
        // For each column in the grid
        for x in 0..game_board.width {
            // If there is a removable piece left
            if game_board.board[y][x] > 0 {
                return false;
            }
        }
    }

    true
}


// Checks one of the ways the puzzle could be impossible early
// Checks to make sure there isn't exactly 1 or 2 of a color
fn check_for_loss(game_board: &GameBoard) -> bool {
    // Array for colors 1-10
    // (There is a spot for 0 even though it is unused to cut down on needed operations)
    let mut color_counter = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    // For each row in the grid
    for y in 0..game_board.height {
        // For each column in the grid
        for x in 0..game_board.width {
            // If it is a removable piece
            if game_board.board[y][x] > 0 {
                // If there is air below it
                color_counter[game_board.board[y][x] as usize] += 1;
            }
        }
    }

    // Make sure none are 1 or 2
    for i in 1..11 {
        if color_counter[i] == 1 || color_counter[i] == 2 {
            return true;
        }
    }

    false
}