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

struct Map {
    width: u32,
    height: u32,
    points: Vec<Vec<u32>>,
}

fn parse_line(line: &str) -> Result<Vec<u32>, ParseError> {
    Ok(line
        .chars()
        .map(|c| c.to_digit(10).ok_or_else(|| ParseError {}))
        .collect::<Result<Vec<_>, _>>()?)
}

enum Neighborhood {
    Corner([(usize, usize); 2]),
    Border([(usize, usize); 3]),
    Inside([(usize, usize); 4]),
}

impl Neighborhood {
    fn points(&self) -> &[(usize, usize)] {
        match self {
            Neighborhood::Corner(data) => data,
            Neighborhood::Border(data) => data,
            Neighborhood::Inside(data) => data,
        }
    }
}

impl Map {
    fn new<B: BufRead>(lines: &mut Lines<B>) -> Result<Self, ParseError> {
        let points = lines
            .map(|l| l.map(|l| parse_line(&l)))
            .flatten()
            .collect::<Result<Vec<Vec<u32>>, ParseError>>()?;

        let width = points[0].len() as u32;
        let height = points.len() as u32;

        Ok(Self {
            width,
            height,
            points,
        })
    }

    fn neighborhood(&self, x: usize, y: usize) -> Neighborhood {
        let max_x = (self.width - 1) as usize;
        let max_y = (self.height - 1) as usize;

        match (x, y) {
            (0, 0) => Neighborhood::Corner([(0, 1), (1, 0)]),
            (x, y) if (x, y) == (max_x, max_y) => Neighborhood::Corner([(x, y - 1), (x - 1, y)]),
            (x, 0) if x == max_x => Neighborhood::Corner([(x - 1, 0), (x, 1)]),
            (x, 0) => Neighborhood::Border([(x - 1, 0), (x, 1), (x + 1, 0)]),
            (0, y) if y == max_y => Neighborhood::Corner([(0, y - 1), (1, y - 1)]),
            (0, y) => Neighborhood::Border([(0, y - 1), (1, y), (0, y + 1)]),
            (x, y) if y == max_y => Neighborhood::Border([(x - 1, y), (x + 1, y), (x, y - 1)]),
            (x, y) if x == max_x => Neighborhood::Border([(x - 1, y), (x, y - 1), (x, y + 1)]),
            (x, y) => Neighborhood::Inside([(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]),
        }
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let p = self.points[y][x];

        self.neighborhood(x, y)
            .points()
            .iter()
            .all(|(x, y)| p < self.points[*y][*x])
    }

    fn low_points_and_heights(&self) -> Vec<(usize, usize, u32)> {
        (0..self.width)
            .flat_map(|x| {
                (0..self.height).filter_map(move |y| {
                    let (x, y) = (x as usize, y as usize);

                    if self.is_low_point(x, y) {
                        Some((x, y, self.points[y][x]))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()
    }

    fn basin_size(&self, x: usize, y: usize) -> usize {
        let mut remaining = vec![(x, y)];
        let mut marked: Vec<(usize, usize)> = vec![];

        while !remaining.is_empty() {
            let mut next = vec![];

            for p in &remaining {
                for (x, y) in self.neighborhood(p.0, p.1).points() {
                    let (x, y) = (*x, *y);
                    let height = self.points[y][x];

                    if height < 9 && !marked.contains(&(x, y)) {
                        next.push((x, y));
                        marked.push((x, y));
                    }
                }
            }

            remaining = next;
        }

        marked.iter().count()
    }
}

fn solve_part_one(map: &Map) -> u32 {
    map.low_points_and_heights().iter().map(|p| p.2 + 1).sum()
}

fn solve_part_two(map: &Map) -> usize {
    let mut sizes = map
        .low_points_and_heights()
        .iter()
        .map(|p| map.basin_size(p.0, p.1))
        .collect::<Vec<_>>();

    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let map = Map::new(&mut reader.lines())?;
    println!("{}", solve_part_one(&map));
    println!("{}", solve_part_two(&map));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn text_example() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"2199943210
3987894921
9856789892
8767896789
9899965678"#,
        );

        let map = Map::new(&mut cursor.clone().lines())?;
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 5);
        assert!(map.is_low_point(9, 0));

        let low_points = map.low_points_and_heights();
        assert_eq!(low_points.len(), 4);

        assert_eq!(solve_part_one(&map), 15);

        assert_eq!(map.basin_size(0, 0), 3);
        assert_eq!(map.basin_size(9, 0), 9);
        assert_eq!(map.basin_size(2, 2), 14);
        assert_eq!(map.basin_size(6, 4), 9);

        assert_eq!(solve_part_two(&map), 1134);

        Ok(())
    }
}
