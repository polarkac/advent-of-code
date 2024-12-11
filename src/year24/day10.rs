use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 10)
}

pub fn part1(input: &str) -> String {
    let map = HeightMap::from_input(input);
    let total: usize = map
        .count_reachable_tops()
        .iter()
        .map(|(_, tops)| {
            let mut set: HashSet<(usize, usize)> = HashSet::new();
            set.extend(tops);

            set.len()
        })
        .sum();

    total.to_string()
}

pub fn part2(input: &str) -> String {
    let map = HeightMap::from_input(input);
    let total: usize = map
        .count_reachable_tops()
        .iter()
        .map(|(_, tops)| tops.len())
        .sum();

    total.to_string()
}

struct HeightMap {
    map: Vec<Vec<u32>>,
    trail_heads: Vec<(usize, usize)>,
}

impl HeightMap {
    fn from_input(input: &str) -> Self {
        let mut trail_heads = Vec::new();
        let map = input.lines().map(|line| line.trim()).enumerate().fold(
            Vec::new(),
            |mut acc, (y, line)| {
                let new_line = line.chars().enumerate().fold(
                    Vec::new(),
                    |mut acc, (x, c)| {
                        let height = c.to_digit(10).unwrap_or(100);
                        if height == 0 {
                            trail_heads.push((y, x));
                        }
                        acc.push(height);

                        acc
                    },
                );

                acc.push(new_line);

                acc
            },
        );

        Self { map, trail_heads }
    }

    fn count_reachable_tops(
        &self,
    ) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        let mut visited_tops = HashMap::new();
        for head in &self.trail_heads {
            visited_tops.insert(*head, self.try_reach_top(*head));
        }

        visited_tops
    }

    fn try_reach_top(&self, head: (usize, usize)) -> Vec<(usize, usize)> {
        let mut set = Vec::new();
        let current_height = self.map[head.0][head.1];
        if current_height == 9 {
            set.push((head.0, head.1));

            return set;
        }

        let up_pos = Direction::Up.next_position(head, self.size());
        let down_pos = Direction::Down.next_position(head, self.size());
        let left_pos = Direction::Left.next_position(head, self.size());
        let right_pos = Direction::Right.next_position(head, self.size());

        if let Some((y, x)) = up_pos {
            let height = self.map[y][x].checked_sub(1);
            if let Some(height) = height {
                if current_height == height {
                    set.extend(self.try_reach_top((y, x)));
                }
            }
        }
        if let Some((y, x)) = down_pos {
            let height = self.map[y][x].checked_sub(1);
            if let Some(height) = height {
                if current_height == height {
                    set.extend(self.try_reach_top((y, x)));
                }
            }
        }
        if let Some((y, x)) = left_pos {
            let height = self.map[y][x].checked_sub(1);
            if let Some(height) = height {
                if current_height == height {
                    set.extend(self.try_reach_top((y, x)));
                }
            }
        }
        if let Some((y, x)) = right_pos {
            let height = self.map[y][x].checked_sub(1);
            if let Some(height) = height {
                if current_height == height {
                    set.extend(self.try_reach_top((y, x)));
                }
            }
        }

        set
    }

    fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_position(
        &self,
        position: (usize, usize),
        size: (usize, usize),
    ) -> Option<(usize, usize)> {
        let next_position = match self {
            Self::Up => (position.0.checked_sub(1), Some(position.1)),
            Self::Down => (position.0.checked_add(1), Some(position.1)),
            Self::Left => (Some(position.0), position.1.checked_sub(1)),
            Self::Right => (Some(position.0), position.1.checked_add(1)),
        };

        match next_position {
            (Some(y), Some(x)) => {
                if y >= size.0 || x >= size.1 {
                    return None;
                }
                Some((y, x))
            }
            _ => None,
        }
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_str = String::new();
        for line in &self.map {
            for c in line {
                map_str.push_str(&c.to_string());
            }
            map_str.push('\n');
        }

        write!(f, "{map_str}").and_then(|_| write!(f, "{:?}", self.trail_heads))
    }
}

#[cfg(test)]
mod test {
    use crate::year24::day10::part1;

    const INPUT_PREVIEW: &str = "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732";

    #[test]
    fn test_preview_part1() {
        assert_eq!("36", part1(INPUT_PREVIEW));
    }
}
