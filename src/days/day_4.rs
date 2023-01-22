use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let mut total: i32 = 0;
    input.lines().for_each(|line| {
        let re = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
        re.captures_iter(line).for_each(|capture_group| {
            let c1: i32 = capture_group[1].parse::<i32>().unwrap();
            let c2: i32 = capture_group[2].parse::<i32>().unwrap();
            let c3: i32 = capture_group[3].parse::<i32>().unwrap();
            let c4: i32 = capture_group[4].parse::<i32>().unwrap();
            if (c1 <= c3 && c2 >= c4) || (c3 <= c1 && c4 >= c2) {
                total += 1;
            }
        });
    });
    println!("Day 4, part 1: Total: {total}");
    return total;
}

pub fn part_2(input: &str) -> i32 {
    let mut total: i32 = 0;
    input.lines().for_each(|line| {
        let re = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
        re.captures_iter(line).for_each(|capture_group| {
            let c1: i32 = capture_group[1].parse::<i32>().unwrap();
            let c2: i32 = capture_group[2].parse::<i32>().unwrap();
            let c3: i32 = capture_group[3].parse::<i32>().unwrap();
            let c4: i32 = capture_group[4].parse::<i32>().unwrap();
            if (c3 <= c1 && c1 <= c4)
                || (c3 <= c2 && c2 <= c4)
                || (c1 <= c3 && c3 <= c2)
                || (c1 <= c4 && c4 <= c2)
            {
                total += 1;
            }
        });
    });
    println!("Day 4, part 2: Total: {total}");
    return total;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_4_example_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_4_example.txt").expect("Couldn't read file!");
        assert_eq!(2, part_1(&input));
        assert_eq!(4, part_2(&input));
    }

    #[test]
    fn day_4_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_4.txt").expect("Couldn't read file!");
        assert_eq!(424, part_1(&input));
        assert_eq!(804, part_2(&input));
    }
}
