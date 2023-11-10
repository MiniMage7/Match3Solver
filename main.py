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
    global puzzleBoard

    # If the move is out of bounds
    if y2 < 0 or x2 >= puzzleBoard.shape[1]:
        return False

    # If the move is swapping with air or a blocker
    if puzzleBoard[y2, x2] < 1:
        return False

    # Swap the 2 spots on the puzzle board
    tempValue = puzzleBoard[y1, x1]
    puzzleBoard[y1, x1] = puzzleBoard[y2, x2]
    puzzleBoard[y2, x2] = tempValue

    # Check if the move results in any blocks being removed
    isMoveValid = checkIfBlocksRemoved(y1, x1) or checkIfBlocksRemoved(y2, x2)

    # Swap the pieces back
    tempValue = puzzleBoard[y1, x1]
    puzzleBoard[y1, x1] = puzzleBoard[y2, x2]
    puzzleBoard[y2, x2] = tempValue

    return isMoveValid


# Takes an x and y coordinate of the puzzle board
# Checks if that piece should be removed
def checkIfBlocksRemoved(y, x):
    global puzzleBoard

    # If it matches with the 2 blocks above it
    if y - 2 >= 0:
        if puzzleBoard[y - 2, x] == puzzleBoard[y - 1, x] == puzzleBoard[y, x]:
            return True
    # If it matches the block above it and the block below it
    if y - 1 >= 0 and y + 1 < puzzleBoard.shape[0]:
        if puzzleBoard[y - 1, x] == puzzleBoard[y, x] == puzzleBoard[y + 1, x]:
            return True
    # If it matches with the 2 blocks below it
    if y + 2 < puzzleBoard.shape[0]:
        if puzzleBoard[y, x] == puzzleBoard[y + 1, x] == puzzleBoard[y + 2, x]:
            return True

    # If it matches with the 2 blocks to the left of it
    if x - 2 >= 0:
        if puzzleBoard[y, x - 2] == puzzleBoard[y, x - 1] == puzzleBoard[y, x]:
            return True
    # If it matches the block to the right and left of it
    if x - 1 >= 0 and x + 1 < puzzleBoard.shape[1]:
        if puzzleBoard[y, x - 1] == puzzleBoard[y, x] == puzzleBoard[y, x + 1]:
            return True
    # If it matches with the 2 blocks to the right of it
    if x + 2 < puzzleBoard.shape[1]:
        if puzzleBoard[y, x] == puzzleBoard[y, x + 1] == puzzleBoard[y, x + 2]:
            return True

    # If none of the moves resulted in blocks being removed
    return False


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
    # Get what blocks need to be removed
    blocksToRemove = checkWhatBlocksToRemove()  # Return array of 0's and 1's where 1's are blocks to remove
    # If there are blocks to remove
    if not blocksToRemove.any():
        # Remove the blocks
        removeGivenBlocks(blocksToRemove)
        # Make all the blocks fall down
        calculateGravity()
        # Restart this process
        recalculateBoard()


# Iterates over the whole puzzle board and returns what blocks need to be removed
# Return: An 2d array of 0s and 1s where 1s represent the positions where blocks need to be removed
def checkWhatBlocksToRemove():
    global puzzleBoard

    blocksToRemove = np.zeros(puzzleBoard.shape)

    # TODO: Stuff

    return blocksToRemove


# Takes an array of 1's and 0's
# Removes all blocks from the puzzle board where the given array has a 1 in the same position
# Used with checkWhatBlocksToRemove()
def removeGivenBlocks(blocksToRemove):
    global puzzleBoard

    # TODO: Stuff


# Makes all blocks that need to fall down in the puzzle board fall down
def calculateGravity():
    global puzzleBoard

    # TODO: Stuff


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
