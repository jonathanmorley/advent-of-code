use anyhow::Result;

fn reading_column(readings: &[String], index: usize) -> Vec<char> {
    readings
        .iter()
        .map(|reading| reading.chars().nth(index).unwrap())
        .collect()
}

fn bit_counts(readings: &[String], index: usize) -> (usize, usize) {
    let (zeroes, ones): (Vec<char>, Vec<char>) = reading_column(readings, index)
        .iter()
        .partition(|bit| **bit == '0');
    (zeroes.len(), ones.len())
}

fn most_common_bit(readings: &[String], index: usize) -> char {
    let (zeroes, ones) = bit_counts(readings, index);
    if zeroes > ones {
        '0'
    } else {
        '1'
    }
}

fn least_common_bit(readings: &[String], index: usize) -> char {
    let (zeroes, ones) = bit_counts(readings, index);
    if zeroes <= ones {
        '0'
    } else {
        '1'
    }
}

fn gamma(readings: &[String]) -> Result<u32> {
    let mut gamma = String::new();

    for i in 0..readings[0].len() {
        gamma.push(most_common_bit(readings, i));
    }

    u32::from_str_radix(&gamma, 2).map_err(Into::into)
}

fn epsilon(readings: &[String]) -> Result<u32> {
    let mut epsilon = String::new();

    for i in 0..readings[0].len() {
        epsilon.push(least_common_bit(readings, i));
    }

    u32::from_str_radix(&epsilon, 2).map_err(Into::into)
}

pub fn part1(input: &str) -> u64 {
    let input_str: Vec<String> = input.lines().map(ToOwned::to_owned).collect();

    let gamma = gamma(&input_str).unwrap();
    let epsilon = epsilon(&input_str).unwrap();

    gamma as u64 * epsilon as u64
}

fn filter_readings(readings: Vec<String>, index: usize, filter_bit: char) -> Vec<String> {
    readings
        .into_iter()
        .filter(|reading| reading.chars().nth(index).unwrap() == filter_bit)
        .collect()
}

fn oxygen_generator(readings: Vec<String>) -> Result<u32> {
    let mut candidates = readings;

    for i in 0.. {
        let mcb = most_common_bit(&candidates, i);

        candidates = filter_readings(candidates, i, mcb);

        if candidates.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&candidates[0], 2).map_err(Into::into)
}

fn co2_scrubber(readings: Vec<String>) -> Result<u32> {
    let mut candidates = readings;

    for i in 0.. {
        let lcb = least_common_bit(&candidates, i);

        candidates = filter_readings(candidates, i, lcb);

        if candidates.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&candidates[0], 2).map_err(Into::into)
}

pub fn part2(input: &str) -> u64 {
    let input_chars: Vec<String> = input.lines().map(ToOwned::to_owned).collect();

    let oxygen_generator = oxygen_generator(input_chars.clone()).unwrap();
    let co2_scrubber = co2_scrubber(input_chars).unwrap();

    oxygen_generator as u64 * co2_scrubber as u64
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 198);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 230);
    }
}
