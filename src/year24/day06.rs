use std::{collections::HashSet, error::Error, fmt::Display, str::FromStr};

use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 06)
}

pub fn part1(input: &str) -> String {
    let map = Map::from_str(input).unwrap();

    map.simulate_patrol().to_string()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GuardDirection {
    fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn new_position(
        &self,
        old_position: (usize, usize),
    ) -> (Option<usize>, Option<usize>) {
        match self {
            Self::Up => (old_position.0.checked_sub(1), Some(old_position.1)),
            Self::Right => {
                (Some(old_position.0), old_position.1.checked_add(1))
            }
            Self::Down => (old_position.0.checked_add(1), Some(old_position.1)),
            Self::Left => (Some(old_position.0), old_position.1.checked_sub(1)),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
        }
    }
}

impl TryFrom<char> for GuardDirection {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let direction = match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => return Err("Not guard"),
        };

        Ok(direction)
    }
}

struct Map {
    width: usize,
    height: usize,
    obstructions: HashSet<(usize, usize)>,
    guard: ((usize, usize), GuardDirection),
}

impl Map {
    fn simulate_patrol(&self) -> usize {
        let mut sim_guard = self.guard.clone();
        let mut visited_pos: HashSet<(usize, usize)> = HashSet::new();
        visited_pos.insert(sim_guard.0);

        loop {
            let new_position = self.is_past_map(sim_guard.1.new_position(sim_guard.0));
            match new_position {
                None => return visited_pos.len(),
                Some((y, x)) => {
                    if let Some(_) = self.obstructions.get(&(y, x)) {
                        sim_guard.1 = sim_guard.1.rotate_right();
                    } else {
                        sim_guard.0 = (y, x);
                        visited_pos.insert(sim_guard.0);
                    }
                }
            }
        }
    }

    fn is_past_map(&self, position: (Option<usize>, Option<usize>)) -> Option<(usize, usize)> {
        match position {
            (None, _) | (_, None) => return None,
            (Some(y), Some(x)) => {
                if y > self.height - 1 {
                    return None;
                }
                if x > self.width - 1 {
                    return None;
                }

                return Some((y, x));
            }
        }
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();
        let height = lines.len();
        let width = lines[0].len();
        let obstructions: HashSet<(usize, usize)> = lines
            .iter()
            .enumerate()
            .map(|(y, line)| {
                let chars = line
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| {
                        let position = (y, x);
                        if c == '#' {
                            return Some(position);
                        }

                        None
                    })
                    .filter_map(|c| c);

                chars
            })
            .fold(HashSet::new(), |mut map, line_obstructions| {
                map.extend(line_obstructions);

                map
            });
        let mut guard = None;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Ok(direction) = GuardDirection::try_from(c) {
                    guard = Some(((y, x), direction));
                }
            }
        }

        Ok(Self {
            width,
            height,
            obstructions,
            guard: guard.unwrap(),
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_map = String::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                match self.obstructions.get(&(y, x)) {
                    Some(_) => string_map.push('#'),
                    None => {
                        if self.guard.0 == (y, x) {
                            string_map.push(self.guard.1.as_char());
                        } else {
                            string_map.push('.');
                        }
                    }
                }
            }
            if y != self.height - 1 {
                string_map.push('\n');
            }
        }

        write!(f, "{string_map}")
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::year24::day06::{part1, Map};

    use super::GuardDirection;

    #[test]
    fn test_preview_part1() {
        let preview_input = "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...";
        assert_eq!("41", part1(preview_input));
    }

    #[test]
    fn test_map_load() {
        let preview_input = "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...";
        let preview_input: String = preview_input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if !line.is_empty() {
                    return Some(line);
                }

                None
            })
            .collect();
        let map = Map::from_str(&preview_input).unwrap();
        assert_eq!(preview_input, map.to_string())
    }

    #[test]
    fn test_direction_rotation() {
        let direction = GuardDirection::Up;

        let direction = direction.rotate_right();
        assert_eq!(direction, GuardDirection::Right);

        let direction = direction.rotate_right();
        assert_eq!(direction, GuardDirection::Down);

        let direction = direction.rotate_right();
        assert_eq!(direction, GuardDirection::Left);

        let direction = direction.rotate_right();
        assert_eq!(direction, GuardDirection::Up);
    }
}
