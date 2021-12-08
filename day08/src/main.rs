use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Copy, Clone, Debug)]
enum Candidate {
    One([u8; 2]),
    Four([u8; 4]),
    Seven([u8; 3]),
    Eight([u8; 7]),
    UnknownFive([u8; 5]),
    UnknownSix([u8; 6]),
    Invalid,
}

impl Candidate {
    fn data(&self) -> &[u8] {
        match self {
            Candidate::One(x) => x,
            Candidate::Four(x) => x,
            Candidate::Seven(x) => x,
            Candidate::Eight(x) => x,
            Candidate::UnknownFive(x) => x,
            Candidate::UnknownSix(x) => x,
            Invalid => panic!("nononon"),
        }
    }
}

type Output = [Candidate; 4];
type Input = [Candidate; 10];

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse segment")
    }
}

impl Error for ParseError {}

fn parse_signal(signal: &[u8]) -> Candidate {
    match signal.len() {
        2 => Candidate::One([signal[0], signal[1]]),
        4 => Candidate::Four([signal[0], signal[1], signal[2], signal[3]]),
        3 => Candidate::Seven([signal[0], signal[1], signal[2]]),
        7 => Candidate::Eight([
            signal[0], signal[1], signal[2], signal[3], signal[4], signal[5], signal[6],
        ]),
        5 => Candidate::UnknownFive([signal[0], signal[1], signal[2], signal[3], signal[4]]),
        6 => Candidate::UnknownSix([signal[0], signal[1], signal[2], signal[3], signal[4], signal[5]]),
        _ => panic!("impossible")
    }
}

fn parse_line(line: &str) -> Result<(Input, Output), ParseError> {
    let mut split = line.split('|');
    let mut left = split
        .next()
        .ok_or_else(|| ParseError {})?
        .split_whitespace();
    let mut right = split
        .next()
        .ok_or_else(|| ParseError {})?
        .split_whitespace();
    let mut input = [Candidate::Invalid; 10];
    let mut output = [Candidate::Invalid; 4];

    for candidate in input.iter_mut() {
        *candidate = parse_signal(left.next().ok_or_else(|| ParseError {})?.as_bytes());
    }

    for candidate in output.iter_mut() {
        *candidate = parse_signal(right.next().ok_or_else(|| ParseError {})?.as_bytes());
    }

    Ok((input, output))
}

fn parse_lines<B: BufRead>(lines: &mut Lines<B>) -> Result<Vec<(Input, Output)>, ParseError> {
    Ok(lines
        .map(|l| l.map(|l| parse_line(&l)))
        .flatten()
        .collect::<Result<Vec<(Input, Output)>, _>>()?)
}

fn part_one(parsed: &[(Input, Output)]) -> usize {
    parsed
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|c| !matches!(c, Candidate::One(_) | Candidate::Four(_) | Candidate::Seven(_) | Candidate::Eight(_)))
                .count()
        })
        .sum::<usize>()
}

fn decode(input: &Input) {
    let one = input.iter().find(|c| matches!(c, Candidate::One(_))).unwrap();
    let seven = input.iter().find(|c| matches!(c, Candidate::Seven(_))).unwrap();

    let a = match (one, seven) {
        (Candidate::One(one), Candidate::Seven(seven)) => {
            seven.iter().filter(|c| !one.contains(c)).next().unwrap()
        },
        _ => panic!("nono"),
    };

    let mut all = input.iter().map(|i| i.data().iter()).flatten().collect::<Vec<_>>();
    all.sort();

    println!("{:?}", all);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let lines = parse_lines(&mut reader.lines())?;
    println!("{}", part_one(&lines));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_line() -> Result<(), Box<dyn std::error::Error>> {
        let (_input, output) = parse_line(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe",
        )?;

        assert!(matches!(output[0], Candidate::Eight(_)));
        assert!(matches!(output[3], Candidate::Four(_)));
        Ok(())
    }

    #[test]
    fn test_example() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#,
        );

        let lines = parse_lines(&mut cursor.lines())?;
        assert_eq!(part_one(&lines), 26);

        Ok(())
    }

    #[test]
    fn test_decode() -> Result<(), Box<dyn std::error::Error>> {
        let (input, _output) = parse_line("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")?;
        decode(&input);

        Ok(())
    }
}
