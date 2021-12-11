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

enum Neighborhood {
    Corner([(usize, usize); 3]),
    Border([(usize, usize); 5]),
    Inside([(usize, usize); 8]),
}

impl Neighborhood {
    fn points(&self) -> &[(usize, usize)] {
        match self {
            Neighborhood::Corner(data) => data,
            Neighborhood::Border(data) => data,
            Neighborhood::Inside(data) => data,
        }
    }

    fn new(x: usize, y: usize) -> Neighborhood {
        match (x, y) {
            (0, 0) => Neighborhood::Corner([(0, 1), (1, 0), (1, 1)]),
            (0, 9) => Neighborhood::Corner([(0, 8), (1, 8), (1, 9)]),
            (9, 0) => Neighborhood::Corner([(8, 0), (8, 1), (9, 1)]),
            (9, 9) => Neighborhood::Corner([(9, 8), (8, 9), (8, 8)]),
            (x, 0) => {
                Neighborhood::Border([(x - 1, 0), (x - 1, 1), (x, 1), (x + 1, 1), (x + 1, 0)])
            }
            (x, 9) => {
                Neighborhood::Border([(x - 1, 9), (x - 1, 8), (x, 8), (x + 1, 8), (x + 1, 9)])
            }
            (0, y) => {
                Neighborhood::Border([(0, y - 1), (1, y - 1), (1, y), (1, y + 1), (0, y + 1)])
            }
            (9, y) => {
                Neighborhood::Border([(9, y - 1), (8, y - 1), (8, y), (8, y + 1), (9, y + 1)])
            }
            (x, y) => Neighborhood::Inside([
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ]),
        }
    }
}

#[derive(Clone)]
struct Grid {
    energy: [[u8; 10]; 10],
}

impl Grid {
    fn new<B: BufRead>(lines: &mut Lines<B>) -> Result<Self, ParseError> {
        let mut energy = [[0u8; 10]; 10];

        for y in 0..10 {
            let line = lines
                .next()
                .ok_or_else(|| ParseError {})?
                .map_err(|_| ParseError {})?;
            let mut chars = line.chars();

            for x in 0..10 {
                let c = chars.next().ok_or_else(|| ParseError {})?;
                match c {
                    '0'..='9' => energy[x][y] = c.to_digit(10).ok_or_else(|| ParseError {})? as u8,
                    _ => return Err(ParseError {}),
                }
            }
        }

        Ok(Grid { energy })
    }

    fn charged(&self) -> Option<Vec<(usize, usize)>> {
        let charged = (0..10)
            .flat_map(|x| {
                (0..10).filter_map(move |y| {
                    if self.energy[x][y] > 9 {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        if charged.is_empty() {
            None
        } else {
            Some(charged)
        }
    }

    fn step(&mut self) -> u32 {
        let mut flashes = 0;

        for x in 0..10 {
            for y in 0..10 {
                self.energy[x][y] += 1;
            }
        }

        while let Some(charged) = self.charged() {
            for (x, y) in charged {
                flashes += 1;
                self.energy[x][y] = 0;

                for (x, y) in Neighborhood::new(x, y).points() {
                    if self.energy[*x][*y] > 0 {
                        self.energy[*x][*y] += 1;
                    }
                }
            }
        }

        flashes
    }

    fn solve_part_one(&mut self, num_steps: usize) -> u32 {
        (0..num_steps).map(|_| self.step()).sum::<u32>()
    }

    fn solve_part_two(&mut self) -> u32 {
        let mut step = 1;

        while self.step() != 100 {
            step += 1;
        }

        step
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let mut grid = Grid::new(&mut reader.lines())?;
    println!("{}", grid.clone().solve_part_one(100));
    println!("{}", grid.solve_part_two());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn text_example() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#,
        );

        let mut grid = Grid::new(&mut cursor.lines())?;
        assert_eq!(grid.clone().solve_part_one(100), 1656);
        assert_eq!(grid.solve_part_two(), 195);
        Ok(())
    }
}
