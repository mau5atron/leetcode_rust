// LeetCode #200: Number of Islands


// Given an m x n 2D binary grid grid which 
// represents a map of '1's (land) and '0's (water), 
// return the number of islands.

// An island is surrounded by water and is formed by 
// connecting adjacent lands horizontally or vertically. 
// You may assume all four edges of the grid are all surrounded by water.

 
// Example 1:
// Input: grid = [
//   ["1","1","1","1","0"],
//   ["1","1","0","1","0"],
//   ["1","1","0","0","0"],
//   ["0","0","0","0","0"]
// ]
// Output: 1



// Example 2:
// Input: grid = [
//   ["1","1","0","0","0"],
//   ["1","1","0","0","0"],
//   ["0","0","1","0","0"],
//   ["0","0","0","1","1"]
// ]
// Output: 3
 

// Constraints:
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] is '0' or '1'.

struct Sol;
impl Sol {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        // let mut borrowed_grid: &mut Vec<Vec<char>> = grid;
        let mut total_islands = 0;

        let row_len = grid.len();
        let col_len = grid[0].len();

        // lets not do this
        // for (row_idx, row) in grid.to_owned().iter().enumerate(){
        //     for (col_idx, col) in row.iter().enumerate(){
        //         if grid[row_idx][col_idx] == '1' {
        //             total_islands+=1;
        //             self::Sol::dfs(row_len as i32, col_len as i32, &mut grid);
        //         }
        //     }
        // }

        let mut i: usize = 0; // row
        while i < row_len {
            let mut j: usize = 0; // reset col to 0 on each iter
            while j < col_len {
                if grid[i][j] == '1' {
                    total_islands += 1;
                    self::Sol::dfs(i as i32, j as i32, &mut grid);
                }

                j += 1;
            }

            i += 1;
        }

        return total_islands;
    }

    fn dfs(row: i32, col: i32, grid: &mut Vec<Vec<char>>){
        let new_row = grid.len() as i32;
        let new_col = grid[0].len() as i32;

        let directions: Vec<Vec<i32>> = vec![
            vec![0,1],
            vec![1,0],
            vec![0,-1],
            vec![-1,0]
        ];

        if row < 0 || col < 0 || row >= new_row || col >= new_col || grid[row as usize][col as usize] == '0' {
            return;
        }

        grid[row as usize][col as usize] = '0';

        for dir in directions.iter(){
            self::Sol::dfs(row+dir[0], col+dir[1], grid);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Sol;

    #[test]
    pub fn test1(){
        let input_grid: Vec<Vec<char>> = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0']
        ];

        let expected_result: i32 = 1;
        assert_eq!(Sol::num_islands(input_grid), expected_result);
    }

    #[test]
    pub fn test2(){
        let input_grid: Vec<Vec<char>> = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1']
        ];

        let expected_result: i32 = 3;
        assert_eq!(Sol::num_islands(input_grid), expected_result);
    }
}

fn main() {
}
