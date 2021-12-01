use std::fs::File;
use std::io::{BufRead, BufReader};

fn number_of_depth_increases(depths: &[usize]) -> usize {
    depths.windows(2).fold(
        0,
        |acc, window| {
            if window[1] > window[0] {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let lines = reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    println!("{}", number_of_depth_increases(&lines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let test_data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(number_of_depth_increases(&test_data), 7);
    }
}
