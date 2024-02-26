# Match 3 Solver
An algorithm that solves a finite match 3 game to completion.

## To Use the Website Version

The website is hosted at https://www.match3solve.com/.  
The website version of the program can also be used locally by opening the index.html file in a web browser.  
The question mark button in the top right should explain anything else you need.  
All the files in the website folder are used for this.

## To Use the Rust Implementation
If this was a release I would attach a .exe for user convenience, but since I am not happy with the program, I wont be doing that here.  
It can be compiled normally with `cargo build` in the /rust directory and then by running the .exe in the /rust/target/debug folder.  

### Input
You can get an input json string from the website by making the board you want to solve and then clicking export. If you use the live website, it currently copies new lines so you'll have to remove those before pasting. The website on this branch has the newlines removed for your convenience. Just paste the exported string directly from your clipboard into the prompt when asked.

### Output
The program wil output a series of swaps in `y1, x1 with y2, x2` format. The top left corner is (0,0). And the tile to the right of that is (0,1). The rest can be easily extrapolated from there.