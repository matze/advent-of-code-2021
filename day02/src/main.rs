use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl From<&Command> for Vector {
    fn from(command: &Command) -> Self {
        match command {
            Command::Forward(x) => Vector::new(*x, 0),
            Command::Up(x) => Vector::new(0, -(*x)),
            Command::Down(x) => Vector::new(0, *x),
        }
    }
}

#[derive(Debug, Clone)]
struct ParseLineError {
    cause: String,
}

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse line: {}", self.cause)
    }
}

impl ParseLineError {
    fn new(cause: String) -> Self {
        Self { cause }
    }
}

impl Error for ParseLineError {}

impl TryFrom<String> for Command {
    type Error = ParseLineError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut split = s.split(' ');

        let command = split
            .next()
            .ok_or_else(|| Self::Error::new("No split point found".to_string()))?;

        let distance: isize = split
            .next()
            .ok_or_else(|| Self::Error::new("No second split".to_string()))?
            .parse()
            .map_err(|err| Self::Error::new(format!("Distance is not a number: {}", err)))?;

        match command {
            "forward" => Ok(Command::Forward(distance)),
            "up" => Ok(Command::Up(distance)),
            "down" => Ok(Command::Down(distance)),
            _ => Err(Self::Error::new(format!(
                "`{}' is not a valid command",
                command
            ))),
        }
    }
}

fn track(commands: &[Command]) -> Vector {
    commands
        .iter()
        .fold(Vector::new(0, 0), |acc, p| acc + p.into())
}

fn aim(commands: &[Command]) -> Vector {
    let mut aim = 0;
    let mut position = Vector::new(0, 0);

    for command in commands {
        match command {
            Command::Down(x) => aim += x,
            Command::Up(x) => aim -= x,
            Command::Forward(x) => {
                position = position + Vector::new(*x, x * aim);
            }
        };
    }

    position
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let commands = reader
        .lines()
        .map(|line| line.map(|line| line.try_into()))
        .flatten()
        .collect::<Result<Vec<Command>, _>>()?;

    let position = track(&commands);
    println!("{}", position.x * position.y);

    let position = aim(&commands);
    println!("{}", position.x * position.y);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let v = Vector::new(0, 1) + Vector::new(0, -1);
        assert_eq!(v, Vector::new(0, 0));
    }

    #[test]
    fn test_line_parser() {
        let parsed = Command::try_from("forward 5".to_string()).unwrap();
        assert!(matches!(parsed, Command::Forward(5)));
    }

    #[test]
    fn test_example() {
        let example = [
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        let position = track(&example);
        assert_eq!(position.x * position.y, 150);
    }

    #[test]
    fn test_aim() {
        let example = [
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        let position = aim(&example);
        assert_eq!(position.x * position.y, 900);
    }
}
