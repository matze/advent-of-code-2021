use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt;
use std::default::Default;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Clone, Debug, PartialEq)]
struct BitCounts<const N: usize> {
    data: [usize; N],
}

impl<const N: usize> BitCounts<N> {
    fn invert(&self) -> Self {
        let mut data = [0; N];

        for (i, v) in self.data.iter().enumerate() {
            data[i] = if v > &0 { 0 } else { 1 };
        }

        Self { data }
    }
}

impl<const N: usize> Default for BitCounts<N> {
    fn default() -> Self {
        Self { data: [0; N] }
    }
}

impl<const N: usize> From<[usize; N]> for BitCounts<N> {
    fn from(data: [usize; N]) -> Self {
        Self { data }
    }
}

impl<const N: usize> Add for BitCounts<N> {
    type Output = BitCounts<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [0; N];

        for ((dest, l), r) in data.iter_mut().zip(&self.data).zip(&rhs.data) {
            *dest = l + r;
        }

        Self { data }
    }
}

impl<const N: usize> From<BitCounts<N>> for usize {
    fn from(bits: BitCounts<N>) -> Self {
        bits.data
            .iter()
            .enumerate()
            .map(|(i, v)| if *v > 0 { 1 << (N - i - 1) } else { 0 })
            .fold(0, |acc, x| acc | x)
    }
}

#[derive(Debug)]
struct BitCountParseError {}

impl fmt::Display for BitCountParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse bitcount string")
    }
}

impl Error for BitCountParseError {}

impl<const N: usize> TryFrom<&String> for BitCounts<N> {
    type Error = BitCountParseError;

    fn try_from(s: &String) -> Result<Self, BitCountParseError> {
        if s.len() != N {
            return Err(BitCountParseError {});
        }

        let mut data = [0; N];

        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => data[i] = 0,
                '1' => data[i] = 1,
                _ => return Err(BitCountParseError {}),
            }
        }

        return Ok(Self { data });
    }
}

fn sum<const N: usize>(lines: &[String]) -> Result<BitCounts<N>, BitCountParseError> {
    let converted = lines
        .iter()
        .map(|line| line.try_into())
        .collect::<Result<Vec<_>, BitCountParseError>>()?;

    let sums = converted
        .into_iter()
        .fold(BitCounts::<N>::default(), |acc, x| acc + x);

    Ok(sums)
}

fn common_bits<const N: usize>(lines: &[String]) -> Result<BitCounts<N>, BitCountParseError> {
    let sums = sum::<N>(lines)?;
    let mut result = BitCounts::default();
    let half_num = lines.len() / 2;

    for (i, v) in sums.data.into_iter().enumerate() {
        result.data[i] = if v > half_num { 1 } else { 0 };
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;
    let gamma_rate_count = common_bits::<12>(&lines)?;
    let gamma_rate: usize = gamma_rate_count.clone().into();
    let epsilon_rate_count = gamma_rate_count.invert();
    let epsilon_rate: usize = epsilon_rate_count.into();
    println!("{}", gamma_rate * epsilon_rate);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bitcount() {
        let bcs: BitCounts<4> = BitCounts::try_from(&"1001".to_string()).unwrap();
        assert_eq!(bcs.data[0], 1);
        assert_eq!(bcs.data[1], 0);
        assert_eq!(bcs.data[2], 0);
        assert_eq!(bcs.data[3], 1);
    }

    #[test]
    fn bitcount_sum() {
        let sum = sum::<4>(&["0110".to_string(), "1010".to_string()]).unwrap();
        let expected = BitCounts::from([1, 1, 2, 0]);
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_common_bits() {
        let bits = common_bits::<5>(&[
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ])
        .unwrap();

        assert_eq!(bits.data[0], 1);
        assert_eq!(bits.data[1], 0);
        assert_eq!(bits.data[2], 1);
        assert_eq!(bits.data[3], 1);
        assert_eq!(bits.data[4], 0);

        let x: usize = bits.clone().into();
        assert_eq!(x, 22);

        let inverted = bits.invert();
        let y: usize = inverted.into();
        assert_eq!(y, 9);
        assert_eq!(x * y, 198);
    }
}
