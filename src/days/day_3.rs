fn get_char_value(c: &char) -> i32 {
    const LOWERCASE_OFFSET: u32 = 96;
    const UPPERCASE_OFFSET: u32 = 64;
    const ALPHABET_SIZE: u32 = 26;

    if c.is_lowercase() {
        return ((*c as u32) - LOWERCASE_OFFSET) as i32;
    } else if c.is_uppercase() {
        return ((*c as u32) - UPPERCASE_OFFSET + ALPHABET_SIZE) as i32;
    } else {
        println!("Major catastrophic error!");
        return 0;
    }
}

pub fn part_1(input: &String) -> i32 {
    let mut total: i32 = 0;
    input.lines().for_each(|line: &str| {
        let (first, second) = line.split_at(line.len() / 2);

        let seen_char: std::collections::HashSet<char> =
            std::collections::HashSet::from_iter(first.chars());

        for c in second.chars() {
            match seen_char.contains(&c) {
                true => {
                    total += get_char_value(&c);
                    break;
                }
                false => (),
            }
        }
    });
    println!("Day 3, part 1: Total: {total}");
    return total;
}

pub fn part_2(input: &String, group_size: i32) -> i32 {
    let mut total: i32 = 0;
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(group_size as usize)
        .for_each(|group| {
            let seen: std::collections::HashSet<char> =
                std::collections::HashSet::from_iter(group[0].chars());

            let seen = group.iter().skip(1).fold(seen, |acc, line| {
                let second: std::collections::HashSet<char> =
                    std::collections::HashSet::from_iter(line.chars());
                acc.intersection(&second).cloned().collect()
            });

            seen.iter().for_each(|c| total += get_char_value(c));
        });
    println!("Day 3, part 2: Total: {total}");
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    const GROUP_SIZE: i32 = 3;

    #[test]
    fn day_3_example_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_3_example.txt").expect("Couldn't read file!");
        assert_eq!(157, part_1(&input));
        assert_eq!(70, part_2(&input, GROUP_SIZE));
    }

    #[test]
    fn day_3_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_3.txt").expect("Couldn't read file!");
        assert_eq!(8176, part_1(&input));
        assert_eq!(2689, part_2(&input, GROUP_SIZE));
    }
}
