use std::collections::HashSet;
use std::fmt;
use std::fs;

fn main() {
    let sudokus = read_sudokus();

    for mut sudoku in sudokus {
        print!("{}", sudoku);
        sudoku = solve(sudoku);
        println!("Valid: {}", sudoku.is_valid());
        println!("Complete: {}", sudoku.is_complete());
        print!("{}", sudoku);
        println!("---------------------------");
    }
}

fn solve(mut sudoku: Sudoku) -> Sudoku {
    if sudoku.is_complete() {
        return sudoku;
    }

    let options = sudoku.get_options();
    if options.len() == 0 {
        return sudoku;
    }
    let option = &options[0];

    for &n in option.1.iter() {
        sudoku.grid[option.0 .0][option.0 .1] = n;

        if sudoku.is_valid() {
            sudoku = solve(sudoku);

            if sudoku.is_complete() {
                return sudoku;
            }
        }
    }
    sudoku.grid[option.0 .0][option.0 .1] = 0;
    return sudoku;
}

#[derive(Debug)]
pub struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let grid = &self.grid;

        for row in grid.iter() {
            write!(f, "{:?}\n", row)?
        }
        return Ok(());
    }
}

impl Sudoku {
    fn get_options(&self) -> Vec<((usize, usize), HashSet<u8>)> {
        let mut options = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &n) in row.iter().enumerate() {
                let mut numbers = HashSet::new();
                if n == 0 {
                    for v in 1..10u8 {
                        numbers.insert(v);
                    }

                    //rows
                    for x in row {
                        numbers.remove(x);
                    }

                    //columns
                    for y in 0..9 {
                        numbers.remove(&self.grid[y][j]);
                    }

                    //subgrids
                    for row in &self.grid[(i / 3) * 3..(i / 3) * 3 + 3] {
                        for z in &row[(j / 3) * 3..(j / 3) * 3 + 3] {
                            numbers.remove(z);
                        }
                    }
                    options.push(((i, j), numbers));
                }
            }
        }
        options.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        return options;
    }

    fn is_complete(&self) -> bool {
        for row in self.grid {
            if row.contains(&0) {
                return false;
            }
        }
        return true;
    }

    fn is_valid(&self) -> bool {
        for i in 0..9 {
            for v in 1..10 {
                // check rows
                let mut rvc = 0u8;
                for &n in self.grid[i].iter() {
                    if v == n {
                        rvc += 1;
                    }
                }
                if rvc > 1 {
                    return false;
                }

                // check columns
                let mut cvc = 0u8;
                for row in self.grid.iter() {
                    if v == row[i] {
                        cvc += 1;
                    }
                }
                if cvc > 1 {
                    return false;
                }
            }
        }

        // check subgrids
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                for v in 1..10 {
                    let mut vc = 0;
                    for row in &self.grid[i..i + 3] {
                        for &n in &row[j..j + 3] {
                            if v == n {
                                vc += 1;
                            }
                        }
                        if vc > 1 {
                            return false;
                        }
                    }
                }
            }
        }

        return true;
    }
}

pub fn read_sudokus() -> Vec<Sudoku> {
    let contents = fs::read_to_string("puzzles.txt").expect("Unable to read file.");
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    let mut grids: Vec<Sudoku> = Vec::new();

    for line in (0..lines.len()).step_by(10) {
        let values = &lines[line + 1..line + 10];

        let mut sudoku = Sudoku {
            grid: [[0u8; 9]; 9],
        };

        for (i, row) in values.iter().enumerate() {
            for (j, col) in row.chars().enumerate() {
                sudoku.grid[i][j] = col.to_string().parse().unwrap();
            }
        }

        grids.push(sudoku)
    }

    // grids.push(Sudoku {
    //     grid: [
    //         [1, 4, 3, 5, 2, 7, 6, 8, 0],
    //         [9, 2, 4, 3, 6, 5, 7, 0, 1],
    //         [2, 3, 1, 8, 5, 6, 4, 7, 0],
    //         [3, 5, 8, 1, 4, 2, 9, 6, 7],
    //         [7, 1, 9, 4, 3, 0, 0, 2, 8],
    //         [4, 9, 6, 7, 0, 8, 2, 1, 3],
    //         [0, 7, 2, 6, 8, 9, 5, 3, 4],
    //         [8, 6, 7, 2, 0, 3, 1, 4, 9],
    //         [0, 8, 5, 9, 1, 4, 3, 0, 2],
    //     ],
    // });

    grids
}
