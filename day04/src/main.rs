use std::io::BufReader;
use std::fs::File;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::io::{BufRead, Lines};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    Marked(usize),
    Unmarked(usize),
}

impl Entry {
    fn number(&self) -> usize {
        match self {
            Entry::Marked(x) => *x,
            Entry::Unmarked(x) => *x,
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry::Unmarked(0)
    }
}

struct Board<const N: usize> {
    entries: [[Entry; N]; N],
}

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse board")
    }
}

impl Error for ParseError {}

impl<const N: usize> Board<N> {
    fn try_from<B: BufRead>(lines: &mut Lines<B>) -> Result<Self, ParseError> {
        let mut entries = [[Entry::default(); N]; N];

        for i in 0..N {
            if let Some(line) = lines.next() {
                let line = line.map_err(|_| ParseError {})?;
                let mut split = line.split_whitespace();

                for j in 0..N {
                    if let Some(number) = split.next() {
                        let number: usize = number.parse().map_err(|_| ParseError {})?;
                        entries[i][j] = Entry::Unmarked(number);
                    } else {
                        return Err(ParseError {});
                    }
                }
            } else {
                return Err(ParseError {});
            }
        }

        Ok(Board { entries })
    }

    fn mark(&mut self, number: usize) {
        self.entries
            .iter_mut()
            .flat_map(|r| r.iter_mut())
            .filter(|e| e.number() == number)
            .for_each(|e| *e = Entry::Marked(number));
    }

    fn complete(&self) -> bool {
        // Cool!
        for row in self.entries.iter() {
            if row.iter().all(|&e| matches!(e, Entry::Marked(_))) {
                return true;
            }
        }

        // Not so cool :-(
        for col in 0..N {
            let mut complete = true;

            for row in 0..N {
                if matches!(self.entries[row][col], Entry::Unmarked(_)) {
                    complete = false;
                }
            }

            if complete {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> usize {
        self.entries
            .iter()
            .flat_map(|r| r.iter())
            .map(|&e| match e {
                Entry::Unmarked(x) => x,
                _ => 0,
            })
            .sum()
    }
}

struct Puzzle<const N: usize>;

impl<const N: usize> Puzzle<N> {
    fn process_bingo<B: BufRead>(lines: &mut Lines<B>) -> Result<(usize, usize), ParseError> {
        let input: Vec<usize> = lines
            .next()
            .ok_or_else(|| ParseError {})?
            .map_err(|_| ParseError {})?
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseError {})?;

        let mut boards: Vec<Board<N>> = vec![];

        loop {
            if let Some(_) = lines.next() {
                boards.push(Board::try_from(lines)?);
            }
            else {
                break;
            }
        }

        let mut sums = vec![];

        for number in input {
            for board in boards.iter_mut().filter(|b| !b.complete()) {
                board.mark(number);

                if board.complete() {
                    sums.push(number * board.unmarked_sum());
                }
            }
        }

        Ok((sums[0], *sums.last().unwrap()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    println!("{:?}", Puzzle::<5>::process_bingo(&mut reader.lines())?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_board() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"#,
        );
        let mut lines = cursor.lines();
        let mut board: Board<5> = Board::try_from(&mut lines)?;
        assert_eq!(board.entries[1][2], Entry::Unmarked(23));

        board.mark(23);
        assert_eq!(board.entries[1][2], Entry::Marked(23));
        assert_eq!(board.unmarked_sum(), 277);

        assert!(!board.complete());
        board.mark(2);
        board.mark(4);
        board.mark(24);
        board.mark(8);
        assert!(board.complete());

        Ok(())
    }

    #[test]
    fn test_example() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#,
        );

        let (winning, last) = Puzzle::<5>::process_bingo(&mut cursor.lines())?;

        assert_eq!(winning, 4512);
        assert_eq!(last, 1924);

        Ok(())
    }
}
