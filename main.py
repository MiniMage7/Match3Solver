import numpy as np

# This can be any shape or size as long as it is rectangular
# 0's are empty space
# -1's are immovable objects
# Any other number is a specific block. Matching #'s are the same block type.
# All block numbers must be positive integers
puzzleBoard = np.array([[1, 2, 3, 0, 0, 0, 0, 0],
                        [7, 7, 7, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0]])

# Stores the solution to the puzzle
movesToSolve = []


# Recursive function that solves the puzzle
def solve():
    global puzzleBoard
    global movesToSolve
    checkForWin()

    # For every row in the puzzle
    for y in range(puzzleBoard.shape[0]):
        # For every column in the puzzle
        for x in range(puzzleBoard.shape[1]):
            # If the piece is a movable piece
            if puzzleBoard[y][x] > 0:
                # Check if the piece can be moved up or right
                # There is no need to check down or left as those would be
                # another piece's up or right respectively
                if checkValidMove(y, x, y - 1, x):  # Up
                    executeMove(y, x, y - 1, x)
                if checkValidMove(y, x, y, x + 1):  # Right
                    executeMove(y, x, y, x + 1)

    # If the puzzle is impossible
    if not movesToSolve:
        print('There is no solution')
        quit()


# Checks if a move is a valid move
# Input coordinates y1, x1 to be swapped with y2, x2
def checkValidMove(y1, x1, y2, x2):
    pass


# Executes a given move on the board
# Input coordinates y1, x1 to be swapped with y2, x2
def executeMove(y1, x1, y2, x2):
    global puzzleBoard
    global movesToSolve

    # Add the move to the move list
    movesToSolve.append([str(y1) + ',' + str(x1), str(y2) + ',' + str(x2)])
    # Save the current board state
    oldBoardState = puzzleBoard.copy()

    # Execute the move and recalculate the new puzzleBoard
    tempValue = puzzleBoard[y1, x1]
    puzzleBoard[y1, x1] = puzzleBoard[y2, x2]
    puzzleBoard[y2, x2] = tempValue
    recalculateBoard()

    # Attempt to do the next move
    solve()

    # If this line was reached, the move was incorrect, so revert to the old board state
    puzzleBoard = oldBoardState
    # And remove the move from the move list
    movesToSolve.pop()


# Check if there are any pieces that need to be removed and removes them
# Then rearranges the board to account for pieces falling
# Recursively calls itself until no pieces move
def recalculateBoard():
    pass


# Checks if the board is in a winning state and if it is, print the solution
def checkForWin():
    global puzzleBoard
    global movesToSolve

    if np.max(puzzleBoard) < 1:
        print('The solution is:')

        for move in movesToSolve:
            print('Swap: ' + move[0] + ' with ' + move[1])

        print('\nMoves are coordinates with 0,0 being the top left, '
              '1,0 being below it, '
              'and 0,1 being to the right of 0,0')

        quit()


# Main
if __name__ == '__main__':
    solve()
