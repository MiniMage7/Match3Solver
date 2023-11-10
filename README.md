# Match 3 Solver
An algorithm that solves a finite match 3 game to completion.

## To Use

### Input
Change the array at the top of the program to be the match 3 board you are wanting solved

+ The board can be any size as long as it is a rectangle/square  
+ 0s are empty space  
+ -1s are immovable tiles that can't be matched  
+ Any positive integer is a normal block that can be matched to be removed. Numbers can only be matched with the same number. You can not swap a block with empty space  

### Output
If there is a way to remove all the blocks on the screen, the output will be a list of moves necessary to do so. An example move would look like "Swap 2,2 with 2,3". These pairs of numbers are cordinates where 0,0 is the top left most corner of the board, 1,0 is the block directly below it, and 0,1 is the block directly to the right of 0,0.