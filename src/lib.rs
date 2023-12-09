use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::Not,
    str::FromStr,
};

use counter::Counter;
use num::Integer;
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

pub fn day4_part1(inp: &str) -> usize {
    inp.lines()
        .map(|line| {
            let scratchcard = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning_nums = scratchcard
                .0
                .split_ascii_whitespace()
                .collect::<HashSet<_>>();
            let nums_you_have = scratchcard
                .1
                .split_ascii_whitespace()
                .collect::<HashSet<_>>();
            match winning_nums.intersection(&nums_you_have).count() {
                0 => 0,
                num_of_winning_nums => 1 << (num_of_winning_nums - 1),
            }
        })
        .sum()
}

pub fn day4_part2(inp: &str) -> usize {
    // Parse cards into a map from card num to (freq, matching numbers)
    let mut card_map = inp
        .lines()
        .enumerate()
        .map(|(iteration_num, line)| {
            let scratchcard = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning_nums = scratchcard
                .0
                .split_ascii_whitespace()
                .collect::<HashSet<_>>();
            let nums_you_have = scratchcard
                .1
                .split_ascii_whitespace()
                .collect::<HashSet<_>>();
            let num_of_matching_nums = winning_nums.intersection(&nums_you_have).count();
            (iteration_num + 1, (1, num_of_matching_nums))
        })
        .collect::<HashMap<_, _>>();
    // Iterate through map in ascending order, modifying freq along the way
    for iteration_num in 1..(card_map.len() + 1) {
        let &(freq, num_of_matching_nums) = card_map.get(&iteration_num).unwrap();
        for amount_to_get_to_next in 1..(num_of_matching_nums + 1) {
            card_map
                .entry(iteration_num + amount_to_get_to_next)
                .and_modify(|c| {
                    *c = (c.0 + freq, c.1);
                });
        }
    }
    // Return sum of all freq
    card_map.values().map(|(freq, _)| freq).sum()
}

pub fn day5_part1(inp: &str) -> u64 {
    // parse seeds as vec and maps as Vec<(u64, u64, u64)>
    let seeds = inp
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    let maps = inp
        .split("\n\n")
        .skip(1)
        .map(|map_str| {
            map_str
                .lines()
                .skip(1)
                .map(|map_line| {
                    let nums = map_line
                        .split(' ')
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<u64>>();
                    (nums[0], nums[1], nums[2])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // for each seed:
    seeds
        .iter()
        .map(|&seed| {
            // run it through each map
            maps.iter().fold(seed, |acc, map| {
                map.iter()
                    .filter_map(|&(dest_range_start, source_range_start, range_len)| {
                        if (source_range_start..(source_range_start + range_len)).contains(&acc) {
                            Some(dest_range_start + (acc - source_range_start))
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap_or(acc)
            })
        })
        // find the min final value
        .min()
        .unwrap()
}

pub fn day5_part2(inp: &str) -> u64 {
    // parse seeds as vec and maps as Vec<(u64, u64, u64)>
    let first_line = inp
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    let maps = inp
        .split("\n\n")
        .skip(1)
        .map(|map_str| {
            map_str
                .lines()
                .skip(1)
                .map(|map_line| {
                    let nums = map_line
                        .split(' ')
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<u64>>();
                    (nums[0], nums[1], nums[2])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let nums = first_line;
    let x = nums
        .iter()
        .enumerate()
        .filter(|(n, _)| n % 2 == 0)
        .map(|(_, v)| v);
    let y = nums
        .iter()
        .enumerate()
        .filter(|(n, _)| n % 2 == 1)
        .map(|(_, v)| v);
    let seeds_iter = x
        .zip(y)
        .flat_map(|(&start, &length)| start..(start + length));
    // for each seed:
    seeds_iter
        .enumerate()
        .map(|(n, seed)| {
            if n % 100_000_000 == 0 {
                println!("On iteration {}", n);
            }
            // run it through each map
            maps.iter().fold(seed, |acc, map| {
                map.iter()
                    .filter_map(|&(dest_range_start, source_range_start, range_len)| {
                        if (source_range_start..(source_range_start + range_len)).contains(&acc) {
                            Some(dest_range_start + (acc - source_range_start))
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap_or(acc)
            })
        })
        // find the min final value
        .min()
        .unwrap()
}

pub fn day6_part1(inp: &str) -> usize {
    let times = inp
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap());
    let distances = inp
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap());
    times
        .zip(distances)
        .map(|(t, d)| {
            let min_num = (t - f64::sqrt(t.powf(2.0) - 4.0 * d)) / 2_f64;
            let max_num = (t + f64::sqrt(t.powf(2.0) - 4.0 * d)) / 2_f64;
            let min_actual_num = if min_num.ceil() == min_num {
                min_num + 1.0
            } else {
                min_num.ceil()
            };
            let max_actual_num = if max_num.floor() == max_num {
                max_num - 1.0
            } else {
                max_num.floor()
            };
            let nums = (min_actual_num as u64)..((max_actual_num as u64) + 1);
            println!("t {} d {} {:?}", t, d, nums);
            nums.count()
        })
        .product()
}

pub fn day6_part2(inp: &str) -> usize {
    let t = inp
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let d = inp
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let min_num = (t - f64::sqrt(t.powf(2.0) - 4.0 * d)) / 2_f64;
    let max_num = (t + f64::sqrt(t.powf(2.0) - 4.0 * d)) / 2_f64;
    let min_actual_num = if min_num.ceil() == min_num {
        min_num + 1.0
    } else {
        min_num.ceil()
    };
    let max_actual_num = if max_num.floor() == max_num {
        max_num - 1.0
    } else {
        max_num.floor()
    };
    let nums = (min_actual_num as u64)..((max_actual_num as u64) + 1);
    println!("t {} d {} {:?}", t, d, nums);
    nums.count()
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum Day7Part1HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for Day7Part1HandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand_counts = s
            .chars()
            .collect::<Counter<_>>()
            .most_common()
            .into_iter()
            .map(|(_, freq)| freq)
            .collect::<Vec<_>>();
        Ok(match hand_counts[..] {
            [5] => Day7Part1HandType::FiveOfAKind,
            [4, 1] => Day7Part1HandType::FourOfAKind,
            [3, 2] => Day7Part1HandType::FullHouse,
            [3, 1, 1] => Day7Part1HandType::ThreeOfAKind,
            [2, 2, 1] => Day7Part1HandType::TwoPair,
            [2, 1, 1, 1] => Day7Part1HandType::OnePair,
            [1, 1, 1, 1, 1] => Day7Part1HandType::HighCard,
            _ => panic!("Unexpected hand type"),
        })
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum Day7Part2HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for Day7Part2HandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand_counts = s
            .chars()
            .filter(|&c| c != 'J')
            .collect::<Counter<_>>()
            .most_common()
            .into_iter()
            .map(|(_, freq)| freq)
            .collect::<Vec<_>>();
        let num_of_jokers = s.chars().filter(|&c| c == 'J').count();
        if hand_counts.is_empty() {
            hand_counts = vec![num_of_jokers];
        } else {
            *(hand_counts.get_mut(0).unwrap()) += num_of_jokers;
        }
        Ok(match hand_counts[..] {
            [5] => Day7Part2HandType::FiveOfAKind,
            [4, 1] => Day7Part2HandType::FourOfAKind,
            [3, 2] => Day7Part2HandType::FullHouse,
            [3, 1, 1] => Day7Part2HandType::ThreeOfAKind,
            [2, 2, 1] => Day7Part2HandType::TwoPair,
            [2, 1, 1, 1] => Day7Part2HandType::OnePair,
            [1, 1, 1, 1, 1] => Day7Part2HandType::HighCard,
            _ => panic!("Unexpected hand type"),
        })
    }
}

#[derive(PartialEq, Eq)]
struct Day7Part1HandAndType {
    hand: String,
    hand_type: Day7Part1HandType,
}

impl FromStr for Day7Part1HandAndType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Day7Part1HandAndType {
            hand: s.to_owned(),
            hand_type: s.parse::<Day7Part1HandType>().unwrap(),
        })
    }
}

impl Ord for Day7Part1HandAndType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => day7_part1_secondary_compare_hands(&self.hand, &other.hand),
            other => other,
        }
    }
}

impl PartialOrd for Day7Part1HandAndType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn day7_part1_secondary_compare_hands(a: &str, b: &str) -> Ordering {
    let card_order = "23456789TJQKA";
    for (aa, bb) in a.chars().zip(b.chars()) {
        match card_order
            .find(aa)
            .unwrap()
            .cmp(&card_order.find(bb).unwrap())
        {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    panic!("Hands are equal. Maybe this is expected. But panicking for now.");
}

#[derive(PartialEq, Eq)]
struct Day7Part2HandAndType {
    hand: String,
    hand_type: Day7Part2HandType,
}

impl FromStr for Day7Part2HandAndType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Day7Part2HandAndType {
            hand: s.to_owned(),
            hand_type: s.parse::<Day7Part2HandType>().unwrap(),
        })
    }
}

impl Ord for Day7Part2HandAndType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => day7_part2_secondary_compare_hands(&self.hand, &other.hand),
            other => other,
        }
    }
}

impl PartialOrd for Day7Part2HandAndType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn day7_part2_secondary_compare_hands(a: &str, b: &str) -> Ordering {
    let card_order = "J23456789TQKA";
    for (aa, bb) in a.chars().zip(b.chars()) {
        match card_order
            .find(aa)
            .unwrap()
            .cmp(&card_order.find(bb).unwrap())
        {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    panic!("Hands are equal. Maybe this is expected. But panicking for now.");
}

struct Day7Part1HandAndBid {
    hand_and_type: Day7Part1HandAndType,
    bid: usize,
}

impl FromStr for Day7Part1HandAndBid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(' ').ok_or("Parsing failed")?;
        Ok(Day7Part1HandAndBid {
            hand_and_type: hand_str.parse::<Day7Part1HandAndType>().unwrap(),
            bid: bid_str.parse().unwrap(),
        })
    }
}

struct Day7Part2HandAndBid {
    hand_and_type: Day7Part2HandAndType,
    bid: usize,
}

impl FromStr for Day7Part2HandAndBid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(' ').ok_or("Parsing failed")?;
        Ok(Day7Part2HandAndBid {
            hand_and_type: hand_str.parse::<Day7Part2HandAndType>().unwrap(),
            bid: bid_str.parse().unwrap(),
        })
    }
}

pub fn day7_part1(inp: &str) -> usize {
    // Parse input into list of (hand, bid)
    let mut hand_and_bid_lst = inp
        .lines()
        .map(|line| line.parse::<Day7Part1HandAndBid>().unwrap())
        .collect::<Vec<_>>();
    // Order list by hand
    hand_and_bid_lst.sort_unstable_by(|a, b| a.hand_and_type.cmp(&b.hand_and_type));
    // Multiply each bid by rank and return sum
    hand_and_bid_lst
        .iter()
        .enumerate()
        .map(|(i, hand_and_bid)| (i + 1) * hand_and_bid.bid)
        .sum()
}

pub fn day7_part2(inp: &str) -> usize {
    // Parse input into list of (hand, bid)
    let mut hand_and_bid_lst = inp
        .lines()
        .map(|line| line.parse::<Day7Part2HandAndBid>().unwrap())
        .collect::<Vec<_>>();
    // Order list by hand
    hand_and_bid_lst.sort_unstable_by(|a, b| a.hand_and_type.cmp(&b.hand_and_type));
    // Multiply each bid by rank and return sum
    hand_and_bid_lst
        .iter()
        .enumerate()
        .map(|(i, hand_and_bid)| (i + 1) * hand_and_bid.bid)
        .sum()
}

enum Day8Direction {
    L,
    R,
}

impl Day8Direction {
    fn from_char(c: &char) -> Day8Direction {
        match c {
            'L' => Day8Direction::L,
            'R' => Day8Direction::R,
            otherwise => panic!("Unexpected char: {}", otherwise),
        }
    }
}

pub fn day8_part1(inp: &str) -> usize {
    // Parse into first line (as LR) and Map<str, (str, str)>
    let (first_str, snd_str) = inp.split_once("\n\n").unwrap();
    let instructions = first_str.chars().map(|c| Day8Direction::from_char(&c));
    let network = snd_str
        .lines()
        .map(|line| {
            let (f, s) = line.split_once(" = ").unwrap();
            let (s_1, s_2) = s[1..s.len() - 1].split_once(", ").unwrap();
            (f, (s_1, s_2))
        })
        .collect::<HashMap<&str, (&str, &str)>>();
    // Cycle through LR, start at AAA, count steps to ZZZ
    let mut curr_node = "AAA";
    for (count, instruction) in instructions.cycle().enumerate() {
        if curr_node == "ZZZ" {
            return count;
        }
        let (left, right) = network.get(curr_node).unwrap();
        curr_node = match instruction {
            Day8Direction::L => left,
            Day8Direction::R => right,
        }
    }
    panic!("Shouldn't get here");
}

pub fn day8_part2(inp: &str) -> u64 {
    // Parse into first line (as LR) and Map<str, (str, str)>
    let (first_str, snd_str) = inp.split_once("\n\n").unwrap();
    let instructions = first_str
        .chars()
        .map(|c| Day8Direction::from_char(&c))
        .collect::<Vec<_>>();
    let network = snd_str
        .lines()
        .map(|line| {
            let (f, s) = line.split_once(" = ").unwrap();
            let (s_1, s_2) = s[1..s.len() - 1].split_once(", ").unwrap();
            (f, (s_1, s_2))
        })
        .collect::<HashMap<&str, (&str, &str)>>();
    let mut curr_nodes = snd_str
        .lines()
        .map(|line| line.split_once(" = ").unwrap().0)
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();
    // This code assumes that each node ending with A will eventually go to a unique node ending with Z.
    // And that after that, the path will loop the same number of iterations to get back to the same Z node.
    // I checked this held for the input we have to run the code on.
    curr_nodes
        .iter_mut()
        .map(|curr_node| {
            for (count, instruction) in instructions.iter().cycle().enumerate() {
                if curr_node.ends_with('Z') {
                    return count as u64;
                }
                let (left, right) = network.get(curr_node).unwrap();
                *curr_node = match instruction {
                    Day8Direction::L => left,
                    Day8Direction::R => right,
                }
            }
            panic!("Shouldn't get here");
        })
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
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

    #[test]
    fn test_day4() {
        let inp = fs::read_to_string("inputs/day4.txt").unwrap();
        assert_eq!(day4_part1(&inp), 23750);
        assert_eq!(day4_part2(&inp), 13261850);
    }

    #[test]
    fn test_day5() {
        let inp = fs::read_to_string("inputs/day5.txt").unwrap();
        assert_eq!(day5_part1(&inp), 389056265);
        assert_eq!(day5_part2(&inp), 137516820);
    }

    #[test]
    fn test_day6() {
        let inp = fs::read_to_string("inputs/day6.txt").unwrap();
        assert_eq!(day6_part1(&inp), 1660968);
        assert_eq!(day6_part2(&inp), 26499773);
    }
    #[test]
    fn test_day7() {
        let inp = fs::read_to_string("inputs/day7.txt").unwrap();
        assert_eq!(day7_part1(&inp), 251545216);
        assert_eq!(day7_part2(&inp), 250384185);
    }

    #[test]
    fn test_day8() {
        let inp = fs::read_to_string("inputs/day8.txt").unwrap();
        assert_eq!(day8_part1(&inp), 20221);
        assert_eq!(day8_part2(&inp), 14616363770447);
    }
}
