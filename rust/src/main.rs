use serde::Deserialize;
use serde_json;
use std::io;
use std::thread;
use std::thread::JoinHandle;

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

    io::stdin().read_line(&mut game_board)
        .expect("Failed to read line.");

    // Format the input into a GameBoard struct
    let game_board: GameBoard = serde_json::from_str(&game_board).unwrap();

    let moves_to_solve : Vec<Swap> = Vec::new();

    solve(game_board, moves_to_solve);
}


// Checks if the board has any valid moves and spawns a new thread to do any found moves
// Those threads will do all the logic for the move and then start from this function again
fn solve(mut game_board: GameBoard, moves_to_solve : Vec<Swap>) -> Vec<Swap> {
    check_for_win();
    if check_for_loss(&game_board) {panic!("The puzzle is in an unsolvable state")};

    // Vector for holding all the handles to spawned threads
    let mut thread_handles : Vec<JoinHandle<()>> = Vec::new();

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

                    // Spawn a new thread to execute the move and continue the process
                    let handle = thread::spawn(move || {
                        execute_move(game_board_copy, Swap{y1:y, x1:x, y2:y + 1, x2:x}, moves_to_solve_copy);
                    });

                    thread_handles.push(handle);
                }
                // Swap Right
                if check_if_valid_move(&mut game_board, Swap{y1:y, x1:x, y2:y, x2:x + 1}) {
                    let game_board_copy = GameBoard{
                        board: game_board.board.clone(),
                        ..game_board
                    };

                    let moves_to_solve_copy = moves_to_solve.to_vec();

                    // Spawn a new thread to execute the move and continue the process
                    let handle = thread::spawn(move || {
                        execute_move(game_board_copy, Swap{y1:y, x1:x, y2:y + 1, x2:x}, moves_to_solve_copy);
                    });

                    thread_handles.push(handle);
                }
            }
        }
    }

    // TODO: Maybe filler
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
fn execute_move(mut game_board: GameBoard, swap: Swap, mut moves_to_solve : Vec<Swap>) -> Vec<Swap> {
    // Swap the 2 spots on the puzzle board
    let temp_value = game_board.board[swap.y1][swap.x1];
    game_board.board[swap.y1][swap.x1] = game_board.board[swap.y2][swap.x2];
    game_board.board[swap.y2][swap.x2] = temp_value;

    // Recalculate the new board as a result of that swap
    game_board = recalculate_board(game_board);

    // Repeat the solving process with the new board
    moves_to_solve = solve(game_board, moves_to_solve);

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
        // TODO:
        // Make all the blocks fall down
        // TODO:
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


// Check if the puzzle is solved and stop the solve if it is
fn check_for_win() {
    // TODO:
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