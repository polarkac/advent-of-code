use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 05)
}

pub fn part1(input: &str) -> String {
    let (rules, updates) = parse_input(input);
    let mut total: u64 = 0;
    for update in updates {
        if is_update_valid(&rules, &update) {
            let half_len = update.len() / 2;
            total += update[half_len] as u64;
        }
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    String::new()
}

fn parse_input(input: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    if let Some((rules, updates)) = input.split_once("\n\n") {
        let mut rules: Vec<(u8, u8)> = rules
            .lines()
            .filter_map(|line| line.split_once("|"))
            .filter_map(|(lvalue, rvalue)| {
                let lvalue = lvalue.trim().parse();
                let rvalue = rvalue.trim().parse();

                if let (Ok(lvalue), Ok(rvalue)) = (lvalue, rvalue) {
                    return Some((lvalue, rvalue));
                }

                None
            })
            .collect();
        rules.sort_by(|p, n| p.1.cmp(&n.1));
        let updates: Vec<Vec<u8>> = updates
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|value| value.trim().parse().unwrap())
                    .collect()
            })
            .collect();

        return (rules, updates);
    }

    (Vec::default(), Vec::default())
}

fn is_update_valid(rules: &[(u8, u8)], update: &[u8]) -> bool {
    let selected_rules: Vec<&(u8, u8)> = rules
        .iter()
        .filter(|rule| (update.contains(&rule.0) && update.contains(&rule.1)))
        .collect();

    for rule in selected_rules {
        let prev_num = rule.0;
        let next_num = rule.1;
        let prev_idx = update.iter().position(|v| *v == prev_num);
        let next_idx = update.iter().position(|v| *v == next_num);

        if prev_idx > next_idx {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use crate::year24::day05::{part1, part2};

    #[test]
    fn test_preview_part1() {
        let preview_input = "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47";
        assert_eq!("143", part1(preview_input));
    }

    #[test]
    fn test_preview_part2() {
        let preview_input = "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47";
        assert_eq!("123", part2(preview_input));
    }

    #[test]
    fn test_fjurty_part1() {
        let preview_input = "84|65
            21|34
            97|66
            12|45
            87|32
            66|17
            53|99
            20|82
            33|44
            10|95
            25|48
            65|21
            49|94
            52|63
            73|35
            82|61
            23|74
            90|80
            49|32
            79|55

            97,12,45,66,17
            24,90,61,80,23
            84,49,65,21,94
            73,22,35
            84,65,21,90,80,12,34,45,58
            61,74,82,49,55,79,35
            44,66,99,44,33,82,45
            97,33,66,53,99,44,17
            87,49,32";
        assert_eq!("375", part1(preview_input));
    }

    #[test]
    fn test_getecko_part1() {
        let preview_input = "21|71
            32|14
            32|74
            71|14
            71|48
            79|99
            48|71
            14|49
            21|14
            46|48
            48|21
            32|78
            99|21
            44|94
            79|99
            74|64
            14|78
            84|44
            48|49
            74|78
            94|99

            21,71,48,49,14
            32,74,14,64,78
            78,46,48,49,21
            79,84,44,94,99
            14,49,78,79,99";
        assert_eq!("184", part1(preview_input));
    }
}
