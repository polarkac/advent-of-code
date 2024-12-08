use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 08)
}

pub fn part1(input: &str) -> String {
    let mut map = Map::from_str(input).unwrap();

    map.calculate_antinodes().to_string()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    y: usize,
    x: usize,
}

struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Position>>,
    antinodes: HashSet<Position>,
}

impl Map {
    fn calculate_antinodes(&mut self) -> usize {
        for (_, positions) in &self.antennas {
            let antinodes: HashSet<Position> =
                Self::calculate_antinodes_for_frequency(positions)
                    .into_iter()
                    .filter(|pos| {
                        pos.x <= self.width - 1 && pos.y <= self.height - 1
                    })
                    .collect();
            self.antinodes.extend(antinodes);
        }

        self.antinodes.len()
    }

    fn calculate_antinodes_for_frequency(
        positions: &[Position],
    ) -> HashSet<Position> {
        let mut antinodes = HashSet::new();
        let mut current_idx = 0;

        while positions.len() > 0 {
            let current_position = &positions[current_idx];
            for position in positions {
                if position == current_position {
                    continue;
                }
                let cp_y = current_position.y as i64;
                let cp_x = current_position.x as i64;
                let p_y = position.y as i64;
                let p_x = position.x as i64;
                let vector_y = cp_y.checked_sub(p_y);
                let vector_x = cp_x.checked_sub(p_x);
                if let (Some(y), Some(x)) = (vector_y, vector_x) {
                    let antinode_y = p_y.checked_sub(y).map(|v| v.try_into());
                    let antinode_x = p_x.checked_sub(x).map(|v| v.try_into());
                    if let (Some(Ok(ay)), Some(Ok(ax))) =
                        (antinode_y, antinode_x)
                    {
                        antinodes.insert(Position { y: ay, x: ax });
                    }
                }
            }
            if current_idx == (positions.len() - 1) {
                break;
            }
            current_idx += 1;
        }

        antinodes
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_str = String::new();
        let antenas = self
            .antennas
            .iter()
            .fold(HashMap::new(), |mut acc, (frequency, positions)| {
                positions.iter().for_each(|pos| {
                    acc.insert(pos, frequency);
                });
                acc
            });
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position { y, x };
                let antena = antenas.get(&position);
                let antinode = self.antinodes.get(&position);
                match (antena, antinode) {
                    (Some(frequency), None) => map_str.push(**frequency),
                    (_, Some(_)) => map_str.push('#'),
                    (None, None) => map_str.push('.'),
                }
            }
            map_str.push('\n');
        }

        write!(f, "{map_str}")
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().map(|line| line.trim()).collect();
        let height = lines.len();
        let width = lines[0].len();
        let antennas: HashMap<char, Vec<Position>> = s
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut cells, (y, line)| {
                line.trim().chars().enumerate().for_each(|(x, c)| {
                    let position = Position { y, x };
                    if c != '.' {
                        cells.entry(c).or_default().push(position);
                    }
                });

                cells
            });

        Ok(Self {
            width,
            height,
            antennas,
            antinodes: HashSet::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{part1, Map};

    const PREVIEW_INPUT: &str = "............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............";

    #[test]
    fn test_preview_part1() {
        assert_eq!("14", part1(PREVIEW_INPUT));
    }

    #[test]
    fn test_antennas_count() {
        let map = Map::from_str(PREVIEW_INPUT).unwrap();
        assert_eq!(
            7,
            map.antennas.iter().fold(0, |mut acc, (_, v)| {
                acc += v.len();

                acc
            })
        )
    }
}
