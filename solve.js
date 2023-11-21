// Javascript intepretation of the Match 3 Solver
// Only usable through the website

// Arrays used for solving
const puzzleBoard = [];
const movesToSolve = [];

// Where output is written out to
const outputBox = document.getElementById("output");

// Function called from the website button to start the solve process
function startSolve() {
    setUpBoard();
    solve();
}

// Sets the puzzleBoard and movesToSolve arrays up pre-solve
function setUpBoard() {
    // Empty the board if it isn't already
    puzzleBoard.length = 0;
    // Empty the solution moves if it isn't already
    movesToSolve.length = 0;

    // Get all the tiles
    let tiles = tileContainer.getElementsByClassName("tile");

    // For each row in the grid
    for (let y = 0; y < height; y++) {
        // For each column in the grid
        const tempRowOfTiles = [];
        for (let x = 0; x < width; x++) {
            // Add that tile's c value to the row of tiles
            const tile = tiles[y * height + x];
            let cNumber = Number(getCNumber(tile));
            tempRowOfTiles.push(cNumber);
        }
        // Add the row of tiles to the puzzleBoard
        puzzleBoard.push(tempRowOfTiles);
    }
}

// Recursive function that solves the puzzle
function solve() {
    checkForWin();

    // For each row in the grid
    for (let y = 0; y < height; y++) {
        // For each column in the grid
        for (let x = 0; x < width; x++) {
            // If the tile is movable
            if (puzzleBoard[y][x] > 0) {
                // Check if the piece can be moved up or right
                // There is no need to check down or left as those would be
                // another piece's up or right respectively
                
                if (checkValidMove(y, x, y - 1, x)) { // Up
                    executeMove(y, x, y - 1, x)
                }
                if (checkValidMove(y, x, y, x + 1)) { // Right
                    executeMove(y, x, y, x + 1)
                }
            }
        }
    }
}

// Checks if a move is a valid move
// Input coordinates y1, x1 to be swapped with y2, x2
function checkValidMove(y1, x1, y2, x2) {

}

// Executes a given move on the board
// Input coordinates y1, x1 to be swapped with y2, x2
function executeMove(y1, x1, y2, x2) {

}


// Checks if any values in the array are greater than 0
// If there are none, the puzzle is solved and calls output solution
function checkForWin() {
    // For each row in the grid
    for (let y = 0; y < height; y++) {
        // For each column in the grid
        for (let x = 0; x < width; x++) {
            // If there is a tile, return
            if (puzzleBoard[y][x] > 0) {
                return;
            }
        }
    }

    // If this line is reached, the puzzle is solved
    outputSolution();
    throw new Error("This is not an error. This is just to stop the solving process on success.");
}

function outputSolution() {
    //TODO:
}