fn sum_calories(input: &String) -> Vec<i32> {
    let calories: Vec<i32> = input
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|x| x.parse::<i32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .collect();
    return calories;
}

pub fn part_1(input: &String) -> i32 {
    let calories: Vec<i32> = sum_calories(input);
    let max_calories: i32 = *calories.iter().max().unwrap();
    println!("The highest amount of calories carried by an elf is {max_calories}!");
    return max_calories;
}

pub fn part_2(input: &String, top_n: i32) -> i32 {
    let mut calories: Vec<i32> = sum_calories(input);
    calories.sort();
    calories.reverse();
    let top_n_calories: i32 = calories.iter().take(top_n as usize).sum();
    println!(
        "The total amount of calories the top {top_n} elves are carrying is {top_n_calories}!"
    );
    return top_n_calories;
}

#[cfg(test)]
mod tests {
    use super::*;
    const TOP_N_ELVES: i32 = 3;

    #[test]
    fn day_1_examples_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_1_example.txt").expect("Couldn't read file!");
        assert_eq!(24000, part_1(&input));
        assert_eq!(45000, part_2(&input, TOP_N_ELVES));
    }

    #[test]
    fn day_1_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_1.txt").expect("Couldn't read file!");
        assert_eq!(71124, part_1(&input));
        assert_eq!(204639, part_2(&input, TOP_N_ELVES));
    }
}
