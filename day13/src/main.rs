use std::collections::HashSet;
use std::convert::TryFrom;
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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl TryFrom<String> for Point {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split(',');
        let first = split.next().ok_or_else(|| ParseError {})?;
        let second = split.next().ok_or_else(|| ParseError {})?;
        Ok(Point {
            x: first.parse().map_err(|_| ParseError {})?,
            y: second.parse().map_err(|_| ParseError {})?,
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

impl TryFrom<String> for Fold {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split('=');
        let first = split.next().ok_or_else(|| ParseError {})?;
        let second = split.next().ok_or_else(|| ParseError {})?;

        match (first, second) {
            ("fold along x", x) => Ok(Fold::X(x.parse().map_err(|_| ParseError {})?)),
            ("fold along y", y) => Ok(Fold::Y(y.parse().map_err(|_| ParseError {})?)),
            _ => Err(ParseError {}),
        }
    }
}

impl Point {
    fn fold(self, fold: Fold) -> Self {
        match fold {
            Fold::X(fx) => {
                if self.x < fx {
                    self
                } else {
                    Point {
                        x: 2 * fx - self.x,
                        y: self.y,
                    }
                }
            }
            Fold::Y(fy) => {
                if self.y < fy {
                    self
                } else {
                    Point {
                        x: self.x,
                        y: 2 * fy - self.y,
                    }
                }
            }
        }
    }
}

fn parse<B: BufRead>(lines: &mut Lines<B>) -> Result<(HashSet<Point>, Vec<Fold>), ParseError> {
    let points = lines
        .take_while(|x| x.as_ref().unwrap().len() > 1)
        .map(|x| x.unwrap().try_into())
        .collect::<Result<HashSet<_>, _>>()?;

    let folds = lines
        .map(|x| x.unwrap().try_into())
        .collect::<Result<Vec<_>, _>>()?;

    Ok((points, folds))
}

fn fold(points: HashSet<Point>, fold: Fold) -> HashSet<Point> {
    points.into_iter().map(|p| p.fold(fold)).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let (mut points, folds) = parse(&mut reader.lines())?;
    println!("{}", fold(points.clone(), folds[0]).len());

    for f in folds {
        points = fold(points, f);
    }

    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;

    for y in 0..height {
        for x in 0..width {
            if points.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn text_example() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#,
        );

        let (points, folds) = parse(&mut cursor.lines())?;
        assert_eq!(points.len(), 18);
        assert_eq!(folds.len(), 2);

        let points = fold(points, folds[0]);
        assert_eq!(points.len(), 17);

        let points = fold(points, folds[1]);
        assert_eq!(points.len(), 16);
        Ok(())
    }
}
