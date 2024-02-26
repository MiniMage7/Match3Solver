# Match 3 Solver
An algorithm that solves a finite match 3 game to completion.

## To Use the Website Version

The website is hosted at https://www.match3solve.com/.  
The website version of the program can also be used locally by opening the index.html file in a web browser.  
The question mark button in the top right should explain anything else you need.  
All the files in the website folder are used for this.

## To Use the Rust Implementation
There is a .exe in the /rust-match-3-solver that you can run.
You can also compile it normally with `cargo build` in the /rust-match-3-solver directory and then by running the .exe in the /rust/target/debug folder.  

### Input
You can get an input JSON string from the website by making the board you want to solve and then clicking export. Just paste the exported string directly from your clipboard into the prompt when asked.

### Output
The program will output a series of swaps in `y1, x1 with y2, x2` format. The top left corner is (0,0). And the tile to the right of that is (0,1). The rest can be easily extrapolated from there.