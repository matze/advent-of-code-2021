use std::io::{BufRead, BufReader, Lines};
use std::fs::File;

enum Line {
    Corrupt(char),
    Incomplete(Vec<char>),
    Good,
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn opening(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn score_incomplete(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score_completion(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn parse_line(s: &str) -> Line {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let last = stack.last();

                if last.is_none() {
                    return Line::Corrupt(c);
                }

                let last = *last.unwrap();

                if last != opening(c) {
                    return Line::Corrupt(c);
                }

                stack.remove(stack.len() - 1);
            }
            _ => panic!(),
        }
    }

    if stack.is_empty() {
        Line::Good
    } else {
        Line::Incomplete(stack)
    }
}

fn parse_lines<B: BufRead>(lines: &mut Lines<B>) -> Result<Vec<Line>, std::io::Error> {
    lines
        .map(|l| l.map(|l| parse_line(&l))).collect::<Result<Vec<_>, _>>()
}

fn solve_part_one(lines: &Vec<Line>) -> usize {
    lines.iter()
        .map(|l| match l {
            Line::Corrupt(c) => score_incomplete(*c),
            _ => 0,
        })
        .sum()
}

fn score_line(line: &Line) -> usize {
    let points: Vec<usize> = match line {
        Line::Incomplete(stack) => {
            stack.iter().rev().map(|&c| score_completion(closing(c))).collect()
        },
        _ => unreachable!(),
    };

    let mut total_score = 0;

    for point in points {
        total_score *= 5;
        total_score += point;
    }

    total_score
}

fn solve_part_two(lines: &Vec<Line>) -> usize {
    let mut scores = lines
        .iter()
        .filter_map(|l| match l {
            Line::Incomplete(_) => Some(score_line(&l)),
            _ => None,
        }).collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let lines = parse_lines(&mut reader.lines())?;
    println!("{}", solve_part_one(&lines));
    println!("{}", solve_part_two(&lines));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_line() {
        assert!(matches!(parse_line("([])"), Line::Good));
        assert!(matches!(parse_line("([]"), Line::Incomplete(_)));
        assert!(matches!(parse_line("([)]"), Line::Corrupt(_)));

        let line = parse_line("{([(<{}[<>[]}>{[]{[(<()>");
        assert!(matches!(line, Line::Corrupt(c) if c == '}'));
    }

    #[test]
    fn text_example() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#,
        );

        let lines = parse_lines(&mut cursor.lines())?;
        assert_eq!(solve_part_one(&lines), 26397);
        assert_eq!(solve_part_two(&lines), 288957);

        Ok(())
    }
}
