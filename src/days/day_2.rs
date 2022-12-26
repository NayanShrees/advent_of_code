fn calculate_total(
    input: &String,
    unit_score: std::collections::HashMap<char, i32>,
    game_score: std::collections::HashMap<&str, i32>,
) -> i32 {
    let mut total_score: i32 = 0;
    input.lines().for_each(|s| {
        match game_score.get(s) {
            Some(n) => total_score += n,
            None => println!("Error in finding game state!"),
        };
        match unit_score.get(&s.chars().last().unwrap()) {
            Some(n) => total_score += n,
            None => println!("Error in finding unit score!"),
        }
    });
    return total_score;
}

pub fn part_1(input: &String) -> i32 {
    let unit_score: std::collections::HashMap<char, i32> =
        std::collections::HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let game_score: std::collections::HashMap<&str, i32> = std::collections::HashMap::from([
        ("A X", 3),
        ("A Y", 6),
        ("A Z", 0),
        ("B X", 0),
        ("B Y", 3),
        ("B Z", 6),
        ("C X", 6),
        ("C Y", 0),
        ("C Z", 3),
    ]);
    let total_score: i32 = calculate_total(input, unit_score, game_score);
    println!("Day 2, part 1: Total score: {total_score}");
    return total_score;
}

pub fn part_2(input: &String) -> i32 {
    let unit_score: std::collections::HashMap<char, i32> =
        std::collections::HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);
    let game_score: std::collections::HashMap<&str, i32> = std::collections::HashMap::from([
        ("A X", 3),
        ("A Y", 1),
        ("A Z", 2),
        ("B X", 1),
        ("B Y", 2),
        ("B Z", 3),
        ("C X", 2),
        ("C Y", 3),
        ("C Z", 1),
    ]);
    let total_score: i32 = calculate_total(input, unit_score, game_score);
    println!("Day 2, part 2: Total score: {total_score}");
    return total_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_example_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_2_example.txt").expect("Couldn't read file!");
        assert_eq!(15, part_1(&input));
        assert_eq!(12, part_2(&input));
    }

    #[test]
    fn day_2_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_2.txt").expect("Couldn't read file!");
        assert_eq!(14531, part_1(&input));
        assert_eq!(11258, part_2(&input));
    }
}
