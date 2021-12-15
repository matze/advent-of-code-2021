use std::collections::HashMap;
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

type Rules = HashMap<Vec<u8>, u8>;

fn parse_rule(line: &str) -> Result<(Vec<u8>, u8), ParseError> {
    let mut split = line.split(' ');
    let input = split.next().ok_or_else(|| ParseError {})?.as_bytes();

    if split.next().ok_or_else(|| ParseError {})? != "->" {
        return Err(ParseError {});
    }

    let output = split
        .next()
        .ok_or_else(|| ParseError {})?
        .chars()
        .next()
        .ok_or_else(|| ParseError {})? as u8;

    Ok((input.iter().map(|c| *c).collect(), output))
}

fn parse<B: BufRead>(lines: &mut Lines<B>) -> Result<(Rules, Vec<u8>), ParseError> {
    let template = lines
        .next()
        .ok_or_else(|| ParseError {})?
        .map_err(|_| ParseError {})?;

    lines
        .next()
        .ok_or_else(|| ParseError {})?
        .map_err(|_| ParseError {})?;

    let mut rules = HashMap::new();

    for line in lines {
        let line = line.map_err(|_| ParseError {})?;
        let (input, output) = parse_rule(&line)?;
        rules.insert(input, output);
    }

    Ok((rules, template.as_bytes().iter().map(|c| *c).collect()))
}

fn step(input: &Vec<u8>, rules: &Rules) -> Vec<u8> {
    let mut result = vec![];
    for window in input.windows(2) {
        result.push(window[0]);
        result.push(*rules.get(window).unwrap());
    }
    result.push(*input.last().unwrap());
    result
}

fn subtract_max_min_quantities(input: &Vec<u8>) -> usize {
    let mut counts = HashMap::new();

    for c in input {
        counts.insert(c, counts.get(c).unwrap_or(&0) + 1);
    }

    let mut pairs = counts.values().collect::<Vec<_>>();
    pairs.sort();
    *pairs.iter().max().unwrap() - *pairs.iter().min().unwrap()
}

fn solve(input: &Vec<u8>, rules: &Rules, num_steps: usize) -> usize {
    let mut input = input.clone();

    for _ in 0..num_steps {
        input = step(&input, rules);
    }

    subtract_max_min_quantities(&input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let (rules, template) = parse(&mut reader.lines())?;
    println!("{}", solve(&template, &rules, 10));
    println!("{}", solve(&template, &rules, 40));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn text_example() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#,
        );

        let (rules, template) = parse(&mut cursor.lines())?;
        assert_eq!(String::from_utf8_lossy(&template), "NNCB");
        assert_eq!(rules.len(), 16);

        let result = step(&template, &rules);
        assert_eq!(String::from_utf8_lossy(&result), "NCNBCHB");

        let result = step(&result, &rules);
        assert_eq!(String::from_utf8_lossy(&result), "NBCCNBBBCBHCB");

        let result = step(&result, &rules);
        assert_eq!(
            String::from_utf8_lossy(&result),
            "NBBBCNCCNBBNBNBBCHBHHBCHB"
        );

        let result = step(&result, &rules);
        assert_eq!(
            String::from_utf8_lossy(&result),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );

        assert_eq!(solve(&template, &rules, 10), 1588);
        assert_eq!(solve(&template, &rules, 40), 2188189693529);
        Ok(())
    }
}
