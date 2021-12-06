use std::fs::File;
use std::io::Read;

fn number_of_fish(initial: &str, num_days: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let mut stock = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for timer in initial.trim().split(',').map(|x| x.parse()) {
        let timer: usize = timer?;
        stock[timer] += 1;
    }

    for _ in 0..num_days {
        let values = stock.clone();
        let xs = 0..9;

        for (i, j) in xs.clone().zip(xs.cycle().skip(1)) {
            stock[i] = values[j];
        }

        stock[6] += values[0];
    }

    Ok(stock.iter().sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open("input")?.read_to_string(&mut input)?;
    println!("{}", number_of_fish(&input, 80)?);
    println!("{}", number_of_fish(&input, 256)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_example() -> Result<(), Box<dyn std::error::Error>> {
        let input = "3,4,3,1,2";
        assert_eq!(number_of_fish(input, 80)?, 5934);
        assert_eq!(number_of_fish(input, 256)?, 26984457539);
        Ok(())
    }
}
