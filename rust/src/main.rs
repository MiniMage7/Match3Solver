use serde::Deserialize;
use serde_json;
use std::io;

#[derive(Deserialize, Debug)]
struct GameBoard {
    height: u8,
    width: u8,
    board: Vec<Vec<u8>>,
}

struct Swap {
    y1: u8,
    x1: u8,
    y2: u8,
    x2: u8,
}


fn main() {
    // Get the game board as a JSON formatted string from the user
    let mut game_board = String::new();

    io::stdin().read_line(&mut game_board)
        .expect("Failed to read line.");

    // Format the input into a GameBoard struct
    let mut game_board: GameBoard = serde_json::from_str(&game_board).unwrap();


}


// Checks if the board has any valid moves and spawns a new thread to do any found moves
// Those threads will do all the logic for the move and then start from this function again
fn solve(mut game_board: GameBoard) {
    check_for_win();
    if check_for_loss() {return};

    // For each row in the grid
    for y in (0..game_board.height){
        // For each column in the grid
        for x in (0..game_board.width){
            // If the tile is movable
            if (game_board.board[y][x] > 0) {
                // Check if the piece can be moved up or left
                // There is no need to check down or right as those would be
                // another piece's up or left respectively

                // Swap Up
                if check_if_valid_move(&mut game_board.board, Swap{y1:y, x1:x, y2:y - 1, x2:x}) {
                    // TODO:
                }
                // Swap Left
                if check_if_valid_move(&game_board.board, Swap{y1:y, x1:x, y2:y, x2:x - 1}) {
                    // TODO:
                }
            }
        }
    }
}


fn check_if_valid_move(board : &Vec<Vec<u8>>, swap: Swap) -> bool {
    // If the swap goes out of bounds
    if swap.y2 < 0 || swap.x2 < 0 {
        return false;
    }

    // If the move is swapping with air or a blocker
    if (board[swap.y2][swap.x2] < 1) {
        return false;
    }

    // Swap the 2 spots on the puzzle board
    let temp_value = board[swap.y1][swap.x1];
    board[swap.y1][swap.x1] = board[swap.y2][swap.x2];
    board[swap.y2][swap.x2] = temp_value;

    let is_move_valid = check_if_blocks_removed(board, (swap.y1, swap.x1)) ||
                                check_if_blocks_removed(board, (swap.y2, swap.x2));

    // Swap the pieces back
    let temp_value = board[swap.y1][swap.x1];
    board[swap.y1][swap.x1] = board[swap.y2][swap.x2];
    board[swap.y2][swap.x2] = temp_value;

    is_move_valid
}


// Takes an x and y coordinate of the puzzle board
// Checks if that piece should be removed
fn check_if_blocks_removed(board : &Vec<Vec<u8>>, tile_location: (u8, u8)) -> bool {
    true
}


// Check if the puzzle is solved and stop the solve if it is
fn check_for_win() {
    // TODO:
}


// Check if the puzzle has an early loss
fn check_for_loss() -> bool {
    // TODO:
    false
}