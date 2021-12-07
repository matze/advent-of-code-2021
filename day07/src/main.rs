use std::fs::File;
use std::io::Read;

fn brute_force<F>(cost_fn: F, pos: &[usize]) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    let lower = *pos.iter().min().unwrap();
    let upper = *pos.iter().max().unwrap();
    let mut best = usize::MAX;

    for i in lower..(upper + 1) {
        let cost = pos.iter().map(|&x| cost_fn(x, i)).sum();

        if cost < best {
            best = cost;
        }
    }

    best
}

fn solve_part_one(pos: &[usize]) -> usize {
    brute_force(|x, y| x.max(y) - x.min(y), pos)
}

fn solve_part_two(pos: &[usize]) -> usize {
    brute_force(
        |x, y| {
            let n = x.max(y) - x.min(y);
            (n * n + n) / 2
        },
        pos,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open("input")?.read_to_string(&mut input)?;

    let input = input
        .trim()
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()?;

    println!("{}", solve_part_one(&input));
    println!("{}", solve_part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_example_part_one() {
        let input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(solve_part_one(&input), 37);
    }

    #[test]
    fn text_example_part_two() {
        let input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(solve_part_two(&input), 168);
    }
}
