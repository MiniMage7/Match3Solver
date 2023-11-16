// Initialize the grid
const gridSizeBoxes = document.getElementsByClassName("sizeinput");
let width = 0;
let height = 0;
updateGridSize();

// Give the input boxes event handlers
gridSizeBoxes[0].children[0].addEventListener("change", updateGridSize);
gridSizeBoxes[1].children[0].addEventListener("change", updateGridSize);

// For determining if the mouse is being held down
let isMouseDown = false;
document.addEventListener("mousedown", mouseDown);
document.addEventListener("mouseup", mouseUp);

// When the grid size boxes are update, adds or removes tiles as needed
function updateGridSize() {
  let oldWidth = width;
  let oldHeight = height;

  // Make sure the changed values are in acceptable ranges
  if (gridSizeBoxes[0].children[0].value > 15) {
    gridSizeBoxes[0].children[0].value = 15;
  } else if (gridSizeBoxes[0].children[0].value < 0) {
    gridSizeBoxes[0].children[0].value = 0; 
  }
  if (gridSizeBoxes[1].children[0].value > 15) {
    gridSizeBoxes[1].children[0].value = 15;
  } else if (gridSizeBoxes[1].children[0].value < 0) {
    gridSizeBoxes[1].children[0].value = 0; 
  }

  width = Number(gridSizeBoxes[0].children[0].value);
  height = Number(gridSizeBoxes[1].children[0].value);

  const tileContainer = document.getElementsByClassName("tilecontainer")[0];

  // Resize grid
  tileContainer.style.gridTemplateColumns = "repeat(" + String(width) + ", auto)";
  tileContainer.style.gridTemplateRows = "repeat(" + String(height) + ", auto)";

  // Add tiles
  if (width * height > oldWidth * oldHeight) {
    for (let i = 0; i < (width * height - oldWidth * oldHeight); i++) {
      const tile = document.createElement("div");
      tile.classList.add("tile", "c0");
      tile.addEventListener("mousedown", cChange);
      tile.addEventListener("mouseenter", mouseEnterTile)
      tileContainer.appendChild(tile);
    }
  }
  // Remove tiles
  else {
    for (let i = 0; i < (oldWidth * oldHeight - width * height); i++) {
      tileContainer.removeChild(tileContainer.lastElementChild);
    }
  }
}

// Changed the class up or down 1 on user click
function cChange(e) {
  let cNumber = Number(getCNumber(e.target));
  let newCNumber = cNumber;

  // If shift is being held go down
  if (e.shiftKey) {
    newCNumber = cNumber - 1;
    if (newCNumber < -1) {
      newCNumber = 10;
    }
  } // Otherwise go up by 1
  else {
    newCNumber = cNumber + 1;
    if (newCNumber > 10) {
      newCNumber = -1;
    }
  }

  e.target.classList.replace("c" + cNumber, "c" + newCNumber);
}

// Returns the value of the current class
function getCNumber(tile) {
  return tile.className.replace("tile", "").replace("c", "").trim()
}

// If the mouse is down on a tile enter, treat it as a click
function mouseEnterTile(e) {
  if (isMouseDown) {
    cChange(e);
  }
}

// Keeps track of if the mouse is being pressed
function mouseDown(e) {
  isMouseDown = true;
}

// Keeps track of if the mouse is being pressed
function mouseUp(e) {
  isMouseDown = false;
}