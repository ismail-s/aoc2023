use std::{collections::HashMap, ops::Not};

use regex::Regex;

pub fn day1_part1(inp: &str) -> u32 {
    return inp
        .lines()
        .map(|line: &str| {
            let i = line.find(|c: char| c.is_numeric()).unwrap();
            let j = line.rfind(|c: char| c.is_numeric()).unwrap();
            let ii = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            let jj = line.chars().nth(j).unwrap().to_digit(10).unwrap();
            ii * 10 + jj
        })
        .sum();
}

pub fn day1_part2(inp: &str) -> u32 {
    let digits_map = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    return inp
        .lines()
        .map(|line: &str| {
            let left_num = digits_map
                .iter()
                .map(|(s, n)| {
                    let res = line.find(s).unwrap_or(usize::MAX);
                    (res, n)
                })
                .min_by(|&(x, _), &(y, _)| x.cmp(&y))
                .unwrap()
                .1;
            let right_num = digits_map
                .iter()
                .map(|(s, n)| {
                    let res = line
                        .rfind(s)
                        .map(|x| i64::try_from(x).unwrap())
                        .unwrap_or(i64::MIN);
                    (res, n)
                })
                .max_by(|&(x, _), &(y, _)| x.cmp(&y))
                .unwrap()
                .1;
            left_num * 10 + right_num
        })
        .sum();
}

pub fn day2_part1(inp: &str) -> usize {
    return inp
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let i = line.find(": ").unwrap() + ": ".len();
            return line[i..].split("; ").all(|sub_game| {
                return sub_game.split(", ").all(|num_and_colour| {
                    let num: u32 = num_and_colour.split(' ').next().unwrap().parse().unwrap();
                    let colour = num_and_colour.split(' ').nth(1).unwrap();
                    match colour {
                        "red" => num <= 12,
                        "green" => num <= 13,
                        "blue" => num <= 14,
                        other => panic!("Unexpected colour: {}", other),
                    }
                });
            });
        })
        .map(|(i, _)| i + 1)
        .sum();
}

pub fn day2_part2(inp: &str) -> u32 {
    return inp
        .lines()
        .map(|line| {
            let i = line.find(": ").unwrap() + ": ".len();
            let nums = line[i..].split("; ").fold((0, 0, 0), |acc, sub_game| {
                return sub_game
                    .split(", ")
                    .fold(acc, |(red, green, blue), num_and_colour| {
                        let num: u32 = num_and_colour.split(' ').next().unwrap().parse().unwrap();
                        let colour = num_and_colour.split(' ').nth(1).unwrap();
                        match colour {
                            "red" => (red.max(num), green, blue),
                            "green" => (red, green.max(num), blue),
                            "blue" => (red, green, blue.max(num)),
                            other => panic!("Unexpected colour: {}", other),
                        }
                    });
            });
            nums.0 * nums.1 * nums.2
        })
        .sum();
}

pub fn day3_part1(inp: &str) -> u32 {
    let lines = inp.lines().collect::<Vec<_>>();
    let num_of_rows = lines.len();
    let num_of_cols = lines[0].len();
    let re = Regex::new("[0-9]+").unwrap();
    re.find_iter(inp)
        .filter(|re_match| {
            let row_match_is_in = re_match.start() / (num_of_cols + 1);
            let match_start_col = re_match.start() % (num_of_cols + 1);
            let match_end_col = match_start_col + re_match.len() - 1;
            let mut cells_to_check = Vec::new();

            if row_match_is_in > 0 {
                (match_start_col..(match_end_col + 1)).for_each(|c| {
                    cells_to_check.push((row_match_is_in - 1, c));
                });
                if match_start_col > 0 {
                    cells_to_check.push((row_match_is_in - 1, match_start_col - 1));
                }
                if match_end_col < (num_of_cols - 1) {
                    cells_to_check.push((row_match_is_in - 1, match_end_col + 1));
                }
            }
            if row_match_is_in < (num_of_rows - 1) {
                (match_start_col..(match_end_col + 1)).for_each(|c| {
                    cells_to_check.push((row_match_is_in + 1, c));
                });
                if match_start_col > 0 {
                    cells_to_check.push((row_match_is_in + 1, match_start_col - 1));
                }
                if match_end_col < (num_of_cols - 1) {
                    cells_to_check.push((row_match_is_in + 1, match_end_col + 1));
                }
            }
            if match_start_col > 0 {
                cells_to_check.push((row_match_is_in, match_start_col - 1));
            }
            if match_end_col < (num_of_cols - 1) {
                cells_to_check.push((row_match_is_in, match_end_col + 1));
            }

            cells_to_check.iter().any(|(r, c)| {
                let symbol = lines[*r].chars().nth(*c).unwrap();
                symbol != '.' && symbol.is_numeric().not()
            })
        })
        .map(|re_match| re_match.as_str().parse::<u32>().unwrap())
        .sum()
}

pub fn day3_part2(inp: &str) -> u32 {
    let lines = inp.lines().collect::<Vec<_>>();
    let num_of_rows = lines.len();
    let num_of_cols = lines[0].len();
    let re = Regex::new("[0-9]+").unwrap();
    let mut gears = HashMap::new();
    re.find_iter(inp).for_each(|re_match| {
        let row_match_is_in = re_match.start() / (num_of_cols + 1);
        let match_start_col = re_match.start() % (num_of_cols + 1);
        let match_end_col = match_start_col + re_match.len() - 1;
        let mut cells_to_check = Vec::new();

        if row_match_is_in > 0 {
            (match_start_col..(match_end_col + 1)).for_each(|c| {
                cells_to_check.push((row_match_is_in - 1, c));
            });
            if match_start_col > 0 {
                cells_to_check.push((row_match_is_in - 1, match_start_col - 1));
            }
            if match_end_col < (num_of_cols - 1) {
                cells_to_check.push((row_match_is_in - 1, match_end_col + 1));
            }
        }
        if row_match_is_in < (num_of_rows - 1) {
            (match_start_col..(match_end_col + 1)).for_each(|c| {
                cells_to_check.push((row_match_is_in + 1, c));
            });
            if match_start_col > 0 {
                cells_to_check.push((row_match_is_in + 1, match_start_col - 1));
            }
            if match_end_col < (num_of_cols - 1) {
                cells_to_check.push((row_match_is_in + 1, match_end_col + 1));
            }
        }
        if match_start_col > 0 {
            cells_to_check.push((row_match_is_in, match_start_col - 1));
        }
        if match_end_col < (num_of_cols - 1) {
            cells_to_check.push((row_match_is_in, match_end_col + 1));
        }

        cells_to_check
            .iter()
            .filter(|(r, c)| {
                let symbol = lines[*r].chars().nth(*c).unwrap();
                symbol == '*'
            })
            .for_each(|&gear| {
                let part_num = re_match.as_str().parse::<u32>().unwrap();
                gears
                    .entry(gear)
                    .and_modify(|lst: &mut Vec<u32>| {
                        lst.push(part_num);
                    })
                    .or_insert(vec![part_num]);
            });
    });
    gears
        .iter()
        .filter(|(_, part_nums)| part_nums.len() == 2)
        .map(|(_, part_nums)| part_nums[0] * part_nums[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_day1() {
        let inp = fs::read_to_string("inputs/day1.txt").unwrap();
        assert_eq!(day1_part1(&inp), 54159);
        assert_eq!(day1_part2(&inp), 53866);
    }

    #[test]
    fn test_day2() {
        let inp = fs::read_to_string("inputs/day2.txt").unwrap();
        assert_eq!(day2_part1(&inp), 2348);
        assert_eq!(day2_part2(&inp), 76008);
    }

    #[test]
    fn test_day3() {
        let inp = fs::read_to_string("inputs/day3.txt").unwrap();
        assert_eq!(day3_part1(&inp), 531932);
        assert_eq!(day3_part2(&inp), 73646890);
    }
}
