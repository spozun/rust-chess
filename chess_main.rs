mod chess_bishop;

fn main() {
  let mut _grid: [[u8; 8]; 8] = [[0; 8]; 8];  //2D array, all 0s
  let mut z = 20;
  //Init the "10x12" chessboard architecture
  //[[21,22,23,24,25,26,27,28],
  // [31,32,33,34,35,36,37,38],
  // [41,42,43,44,45,46,47,48],
  // [51,52,53,54,55,56,57,58],
  // [61,62,63,64,65,66,67,68],
  // [71,72,73,74,75,76,77,78],
  // [81,82,83,84,85,86,87,88],
  // [91,92,93,94,95,96,97,98]]
    for x in 0..8 {
    for y in 0..8 {
      _grid[x][y] = y as u8 + 1 + z;
    }
    z+= 10;
  }
  chess_bishop::find_valid_bishop_moves(85, _grid);
  //outputs 74, 63, 52, 41, 96, 76, 67, 58, 94
}
