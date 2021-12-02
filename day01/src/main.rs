use std::fs::File;
use std::io::{BufRead, BufReader};

fn number_of_depth_increases(depths: &[usize]) -> usize {
    depths.windows(2).filter(|w| w[1] > w[0]).count()
}

fn sums(depths: &[usize]) -> Vec<usize> {
    depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let lines = reader
        .lines()
        .map(|line| line.map(|line| line.parse::<usize>()))
        .flatten()
        .collect::<Result<Vec<usize>, _>>()?;

    println!("{}", number_of_depth_increases(&lines));
    println!("{}", number_of_depth_increases(&sums(&lines)));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_a() {
        let test_data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(number_of_depth_increases(&test_data), 7);
    }

    #[test]
    fn example_part_b() {
        let test_data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let sums = sums(&test_data);
        assert_eq!(sums[0], 607);
        assert_eq!(sums[1], 618);
        assert_eq!(number_of_depth_increases(&sums), 5);
    }
}
