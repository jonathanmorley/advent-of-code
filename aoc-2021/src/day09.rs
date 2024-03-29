use itertools::Itertools;

#[derive(Clone, Debug)]
struct HeightMap(Vec<Vec<u8>>);

impl HeightMap {
    fn get(&self, x: usize, y: usize) -> Option<&u8> {
        self.0.get(x).and_then(|x| x.get(y))
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        vec![
            if x > 0 { Some((x - 1, y)) } else { None },
            if y > 0 { Some((x, y - 1)) } else { None },
            if x < self.0.len() - 1 {
                Some((x + 1, y))
            } else {
                None
            },
            if y < self.0[x].len() - 1 {
                Some((x, y + 1))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn is_local_minimum(&self, x: usize, y: usize) -> bool {
        let value = self.get(x, y).unwrap();

        self.neighbours(x, y)
            .into_iter()
            .map(|(x, y)| self.get(x, y).unwrap())
            .all(|neighbour| neighbour > value)
    }

    fn local_minima(&self) -> Vec<(usize, usize)> {
        (0..self.0.len())
            .flat_map(|x| {
                (0..self.0[x].len()).filter_map(move |y| {
                    if self.is_local_minimum(x, y) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn risk_level(&self, x: usize, y: usize) -> u8 {
        self.get(x, y).unwrap() + 1
    }

    fn flatten(&mut self, x: usize, y: usize) -> bool {
        if self.get(x, y) == Some(&9) {
            false
        } else {
            self.0[x][y] = 9;
            true
        }
    }

    fn extract_basin(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        if self.flatten(x, y) {
            self.neighbours(x, y)
                .iter()
                .flat_map(|(x, y)| self.extract_basin(*x, *y))
                .chain(std::iter::once((x, y)))
                .collect()
        } else {
            Vec::new()
        }
    }

    fn extract_basins(&mut self) -> Vec<Vec<(usize, usize)>> {
        let mut basins = Vec::new();

        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                if self.get(x, y) != Some(&9) {
                    basins.push(self.extract_basin(x, y));
                }
            }
        }

        basins
    }
}

fn generator(input: &str) -> HeightMap {
    HeightMap(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect()
            })
            .collect(),
    )
}

pub fn part1(input: &str) -> u32 {
    let input = generator(input);

    input
        .local_minima()
        .into_iter()
        .map(|(x, y)| input.risk_level(x, y) as u32)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut input = generator(input);

    input
        .extract_basins()
        .into_iter()
        .map(|basin| basin.len() as u64)
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(SAMPLE), 1134);
    }
}
