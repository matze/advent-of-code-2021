use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse segment")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Error for ParseError {}

struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn diagonal(&self) -> bool {
        (self.start.x as isize - self.end.x as isize).abs() == (self.start.y as isize - self.end.y as isize).abs()
            && self.start.x != self.end.x
            && self.start.y != self.end.y
    }
}

impl TryFrom<&str> for Segment {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut split = s.split_whitespace();

        fn parse_tuple(s: &str) -> Result<Point, ParseError> {
            let mut tuple = s.split(',');

            let x = tuple
                .next()
                .ok_or_else(|| ParseError {})?
                .parse()
                .map_err(|_| ParseError {})?;

            let y = tuple
                .next()
                .ok_or_else(|| ParseError {})?
                .parse()
                .map_err(|_| ParseError {})?;

            Ok(Point::new(x, y))
        }

        let start = parse_tuple(split.next().ok_or_else(|| ParseError {})?)?;
        let arrow = split.next().ok_or_else(|| ParseError {})?;
        let end = parse_tuple(split.next().ok_or_else(|| ParseError {})?)?;

        if arrow != "->" {
            return Err(ParseError {});
        }

        Ok(Segment { start, end })
    }
}

impl TryFrom<String> for Segment {
    type Error = ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
    }
}

fn parse_segments<B: BufRead>(lines: &mut Lines<B>) -> Result<Vec<Segment>, ParseError> {
    Ok(lines
        .map(|l| l.map(|l| l.try_into()))
        .flatten()
        .collect::<Result<Vec<Segment>, _>>()?)
}

fn solve<'a, I>(segments: I) -> usize
where
    I: Iterator<Item = &'a Segment>,
{
    let mut acc = HashMap::<Point, usize>::new();

    for Segment { start, end } in segments {
        let dx = if start.x > end.x { -1 } else if start.x < end.x { 1 } else { 0 };
        let dy = if start.y > end.y { -1 } else if start.y < end.y { 1 } else { 0 };

        let mut x = start.x as isize;
        let mut y = start.y as isize;

        while x != end.x as isize || y != end.y as isize {
            *acc.entry(Point::new(x as usize, y as usize)).or_insert(0) += 1;
            x += dx;
            y += dy;
        }

        *acc.entry(Point::new(x as usize, y as usize)).or_insert(0) += 1;
    }

    acc.values().filter(|&c| c >= &2).count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let segments = parse_segments(&mut reader.lines())?;
    println!("{}", solve(segments.iter().filter(|&s| !s.diagonal())));
    println!("{}", solve(segments.iter()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_segment() -> Result<(), Box<dyn std::error::Error>> {
        let segment: Segment = "0,9 -> 5,9".try_into()?;
        assert_eq!(segment.start, Point::new(0, 9));
        assert_eq!(segment.end, Point::new(5, 9));
        Ok(())
    }

    #[test]
    fn test_example() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
        );

        let segments = parse_segments(&mut cursor.lines())?;
        assert_eq!(solve(segments.iter().filter(|&s| !s.diagonal())), 5);
        assert_eq!(solve(segments.iter()), 12);

        Ok(())
    }
}
