use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 04)
}

pub fn part1(input: &str) -> String {
    let chars: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut total: u64 = 0;
    for (y, row) in chars.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &'X' {
                let xmas_count = try_all_directions(&chars, (y, x));
                total += xmas_count;
            }
        }
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    let chars: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut total: u64 = 0;
    for (y, row) in chars.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &'A' {
                if try_x_for_mas(&chars, (y, x)) {
                    total += 1;
                }
            }
        }
    }

    total.to_string()
}

fn try_x_for_mas(chars: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    if pos.0 < 1
        || pos.0 > chars.len() - 1
        || pos.1 < 1
        || pos.1 > chars[0].len() - 1
    {
        return false;
    }
    let diag_left_pos = [
        (pos.0.saturating_sub(1), pos.1.saturating_sub(1)),
        (pos.0, pos.1),
        (pos.0.saturating_add(1), pos.1.saturating_add(1)),
    ];
    let diag_right_pos = [
        (pos.0.saturating_sub(1), pos.1.saturating_add(1)),
        (pos.0, pos.1),
        (pos.0.saturating_add(1), pos.1.saturating_sub(1)),
    ];

    let mut diag_left = Vec::new();
    for pos in diag_left_pos {
        let word = chars.get(pos.0).map(|row| row.get(pos.1)).flatten();
        if let Some(c) = word {
            diag_left.push(c);
        }
    }
    let mut diag_right = Vec::new();
    for pos in diag_right_pos {
        let word = chars.get(pos.0).map(|row| row.get(pos.1)).flatten();
        if let Some(c) = word {
            diag_right.push(c);
        }
    }
    if diag_left.len() == 3 && diag_right.len() == 3 {
        let diag_left = String::from_iter(diag_left);
        let diag_right = String::from_iter(diag_right);

        if (diag_left.as_str() == "MAS" || diag_left.as_str() == "SAM")
            && (diag_right.as_str() == "MAS" || diag_right.as_str() == "SAM")
        {
            return true;
        }
    }

    false
}

fn try_all_directions(chars: &Vec<Vec<char>>, pos: (usize, usize)) -> u64 {
    let mut count = 0;
    let lr_range = pos.1..=pos.1 + 3;
    let rl_range = pos.1.saturating_sub(3)..=pos.1;
    let lr = chars
        .get(pos.0)
        .map(|row| row.get(lr_range).map(|col| String::from_iter(col)))
        .flatten();
    let rl = chars
        .get(pos.0)
        .map(|row| row.get(rl_range).map(|col| String::from_iter(col)))
        .flatten();

    if check_up(chars, &pos) {
        count += 1;
    }
    if check_down(chars, &pos) {
        count += 1;
    }
    if check_diagonal_up_lr(chars, &pos) {
        count += 1;
    }
    if check_diagonal_up_rl(chars, &pos) {
        count += 1;
    }
    if check_diagonal_down_lr(chars, &pos) {
        count += 1;
    }
    if check_diagonal_down_rl(chars, &pos) {
        count += 1;
    }
    if let Some("XMAS") = lr.as_deref() {
        count += 1;
    }
    if let Some("SAMX") = rl.as_deref() {
        count += 1;
    }

    count
}

fn check_from_positions(
    chars: &Vec<Vec<char>>,
    positions: &[(usize, usize)],
) -> bool {
    let mut diag_chars = Vec::new();
    for pos in positions {
        let word = chars.get(pos.0).map(|row| row.get(pos.1)).flatten();
        if let Some(c) = word {
            diag_chars.push(c);
        }
    }
    if diag_chars.len() == 4 {
        let word = String::from_iter(diag_chars);
        if word.as_str() == "XMAS" {
            return true;
        }
    }

    false
}

fn check_up(chars: &Vec<Vec<char>>, pos: &(usize, usize)) -> bool {
    if pos.0 >= 3 {
        let up_pos = [
            (pos.0, pos.1),
            (pos.0.saturating_sub(1), pos.1),
            (pos.0.saturating_sub(2), pos.1),
            (pos.0.saturating_sub(3), pos.1),
        ];

        return check_from_positions(chars, &up_pos);
    }

    false
}

fn check_down(chars: &Vec<Vec<char>>, pos: &(usize, usize)) -> bool {
    let lines_count = chars.len();

    if pos.0 <= lines_count - 3 {
        let down_pos = [
            (pos.0, pos.1),
            (pos.0.saturating_add(1), pos.1),
            (pos.0.saturating_add(2), pos.1),
            (pos.0.saturating_add(3), pos.1),
        ];

        return check_from_positions(chars, &down_pos);
    }

    false
}

fn check_diagonal_up_lr(chars: &Vec<Vec<char>>, pos: &(usize, usize)) -> bool {
    let line_len = chars[0].len();

    if pos.0 >= 3 && pos.1 <= line_len - 3 {
        let diag_up_lr_pos = [
            (pos.0, pos.1),
            (pos.0.saturating_sub(1), pos.1.saturating_add(1)),
            (pos.0.saturating_sub(2), pos.1.saturating_add(2)),
            (pos.0.saturating_sub(3), pos.1.saturating_add(3)),
        ];

        return check_from_positions(chars, &diag_up_lr_pos);
    }

    false
}

fn check_diagonal_up_rl(chars: &Vec<Vec<char>>, pos: &(usize, usize)) -> bool {
    if pos.0 >= 3 && pos.1 >= 3 {
        let diag_up_rl_pos = [
            (pos.0, pos.1),
            (pos.0.saturating_sub(1), pos.1.saturating_sub(1)),
            (pos.0.saturating_sub(2), pos.1.saturating_sub(2)),
            (pos.0.saturating_sub(3), pos.1.saturating_sub(3)),
        ];

        return check_from_positions(chars, &diag_up_rl_pos);
    }

    false
}

fn check_diagonal_down_lr(
    chars: &Vec<Vec<char>>,
    pos: &(usize, usize),
) -> bool {
    let line_len = chars[0].len();
    let lines_count = chars.len();

    if pos.0 <= lines_count - 3 && pos.1 <= line_len - 3 {
        let diag_down_lr_pos = [
            (pos.0, pos.1),
            (pos.0.saturating_add(1), pos.1.saturating_add(1)),
            (pos.0.saturating_add(2), pos.1.saturating_add(2)),
            (pos.0.saturating_add(3), pos.1.saturating_add(3)),
        ];

        return check_from_positions(chars, &diag_down_lr_pos);
    }

    false
}

fn check_diagonal_down_rl(
    chars: &Vec<Vec<char>>,
    pos: &(usize, usize),
) -> bool {
    let lines_count = chars.len();

    if pos.0 <= lines_count - 3 && pos.1 >= 3 {
        let diag_down_rl_pos = [
            (pos.0, pos.1),
            (pos.0.saturating_add(1), pos.1.saturating_sub(1)),
            (pos.0.saturating_add(2), pos.1.saturating_sub(2)),
            (pos.0.saturating_add(3), pos.1.saturating_sub(3)),
        ];

        return check_from_positions(chars, &diag_down_rl_pos);
    }

    false
}

#[cfg(test)]
mod test {
    use crate::year24::day04::{part1, part2};

    #[test]
    fn test_preview_part1() {
        let input_preview = "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            ";
        assert_eq!("18", part1(input_preview));
    }

    #[test]
    fn test_preview_part2() {
        let input_preview = "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            ";
        assert_eq!("9", part2(input_preview));
    }
}
