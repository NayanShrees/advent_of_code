fn get_stack_size(stack_line: &str) -> usize {
    let mut stacks: std::str::Lines = stack_line.lines();
    // 4x - 1 = char count per line
    let total: usize = (stacks.next().unwrap().chars().count() + 1) / 4;
    return total;
}

pub fn part_1(input: &str) -> &str {
    let chunks: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();

    return "dab";
}

pub fn part_2(input: &str) -> &str {
    return "dab";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_5_example_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_5_example.txt").expect("Couldn't read file!");
        //assert_eq!("CMZ", part_1(&input));
        //assert_eq!(4, part_2(&input));
    }

    #[test]
    fn day_5_correct() {
        let input: String =
            std::fs::read_to_string("inputs/day_5.txt").expect("Couldn't read file!");
        //assert_eq!("DAB", part_1(&input));
        //assert_eq!(804, part_2(&input));
    }
}
