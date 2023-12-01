pub fn day1_part1(inp: &str) -> u32 {
    return inp
        .lines()
        .map(|line: &str| {
            let i = line.find(|c: char| c.is_numeric()).unwrap();
            let j = line.rfind(|c: char| c.is_numeric()).unwrap();
            let ii = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            let jj = line.chars().nth(j).unwrap().to_digit(10).unwrap();
            return ii * 10 + jj;
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
                    return (res, n);
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
                    return (res, n);
                })
                .max_by(|&(x, _), &(y, _)| x.cmp(&y))
                .unwrap()
                .1;
            return left_num * 10 + right_num;
        })
        .sum();
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
}
