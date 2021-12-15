#![feature(generic_const_exprs)]

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse line")
    }
}

impl Error for ParseError {}

#[derive(Debug)]
struct Map<const M: usize, const N: usize> {
    grid: [[u32; N]; M],
}

impl<const M: usize, const N: usize> Map<M, N> {
    fn new<B: BufRead>(lines: &mut Lines<B>) -> Result<Self, ParseError> {
        let mut grid = [[0u32; N]; M];

        for (row, line) in lines.enumerate() {
            if row == M {
                println!("A");
                return Err(ParseError {});
            }

            let line = line.map_err(|_| ParseError {})?;

            if line.len() > N {
                return Err(ParseError {});
            }

            for (col, char) in line.chars().enumerate() {
                grid[row][col] = char.to_digit(10).ok_or_else(|| ParseError {})? as u32;
            }
        }

        Ok(Self { grid })
    }

    fn distance_matrix(&self) -> Self {
        let mut grid = [[0u32; N]; M];

        for row in 0..M {
            for col in 0..N {
                grid[row][col] = self.grid[row][col]
                    + match (row, col) {
                        (0, 0) => 0,
                        (0, _) => grid[0][col - 1],
                        (_, 0) => grid[row - 1][0],
                        (_, _) => grid[row][col - 1].min(grid[row - 1][col]),
                    };
            }
        }

        Self { grid }
    }

    fn enlarge(&self) -> Map<{ 5 * M }, { 5 * N }> {
        let mut grid = [[0u32; 5 * N]; 5 * M];

        for row in 0..M {
            for col in 0..N {
                grid[row][col] = self.grid[row][col];
            }
        }

        for tile_row in 0..5 {
            // Extend to the right
            for tile_col in 1..5 {
                for row in 0..M {
                    for col in 0..N {
                        let element = grid[tile_row * M + row][(tile_col - 1) * N + col] + 1;
                        grid[tile_row * M + row][tile_col * N + col] =
                            if element > 9 { 1 } else { element };
                    }
                }
            }

            // Extend first tile column downwards
            if tile_row < 4 {
                for row in 0..M {
                    for col in 0..N {
                        let element = grid[tile_row * M + row][col] + 1;
                        grid[(tile_row + 1) * M + row][col] = if element > 9 { 1 } else { element };
                    }
                }
            }
        }

        Map::<{ 5 * M }, { 5 * N }> { grid }
    }
}

fn solve<const M: usize, const N: usize>(map: &Map<M, N>) -> u32 {
    let d = map.distance_matrix();
    d.grid[M - 1][N - 1] - map.grid[0][0]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let map = Map::<100, 100>::new(&mut reader.lines())?;
    println!("{}", solve::<100, 100>(&map));
    println!("{}", solve::<500, 500>(&map.enlarge()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn text_example() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

        let map = Map::<10, 10>::new(&mut Cursor::new(input).lines())?;
        assert_eq!(solve::<10, 10>(&map), 40);

        let enlarged = map.enlarge();
        assert_eq!(enlarged.grid[10][10], 3);
        assert_eq!(solve::<50, 50>(&enlarged), 315);

        Ok(())
    }
}
