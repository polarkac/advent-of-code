use std::{fmt::Display, str::FromStr};

use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 09)
}

pub fn part1(input: &str) -> String {
    let mut disk_map = DiskMap::from_str(input).unwrap();
    disk_map.defrag();

    disk_map.checksum().to_string()
}

#[derive(Debug, PartialEq, Clone)]
enum Block {
    File(usize),
    Space,
}

struct DiskMap {
    blocks: Vec<Block>,
}

impl DiskMap {
    fn defrag(&mut self) {
        let (mut head, mut tail) = (0, self.blocks.len() - 1);
        while head != tail {
            if let Block::Space = self.blocks[head] {
                self.blocks.swap(head, tail);
                tail -= 1;
            }
            head += 1;
            while let Block::Space = self.blocks[tail] {
                tail -= 1;
            }
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| **block != Block::Space)
            .enumerate()
            .fold(0, |mut acc, (i, block)| {
                if let Block::File(id) = block {
                    acc += i * id;
                }
                acc
            })
    }
}

impl FromStr for DiskMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::new();
        let mut last_file_id = 0;
        for (i, c) in s.trim().chars().enumerate() {
            let block_len = c.to_digit(10).unwrap();
            let block_type = if i % 2 == 1 {
                Block::Space
            } else {
                last_file_id += 1;
                Block::File(last_file_id - 1)
            };
            for _ in 0..block_len {
                blocks.push(block_type.clone());
            }
        }

        Ok(Self { blocks })
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_str = String::new();
        for block in &self.blocks {
            let symbol = match block {
                Block::File(id) => &id.to_string(),
                Block::Space => &'.'.to_string(),
            };
            map_str.push_str(symbol);
        }

        write!(f, "{map_str}")
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{part1, Block, DiskMap};

    const PREVIEW_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_preview_part1() {
        assert_eq!("1928", part1(PREVIEW_INPUT));
    }

    #[test]
    fn test_disk_mapping() {
        let map = DiskMap::from_str(PREVIEW_INPUT).unwrap();

        assert_eq!(Block::Space, map.blocks[2]);
        assert_eq!(&Block::File(9), map.blocks.iter().last().unwrap());
    }

    #[test]
    fn test_disk_defrag() {
        let mut map = DiskMap::from_str(PREVIEW_INPUT).unwrap();
        map.defrag();
        let defrag_map = "0099811188827773336446555566..............";

        assert_eq!(defrag_map, map.to_string());
    }
}
