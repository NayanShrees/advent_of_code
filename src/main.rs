mod days;

fn main() {
    solve_days(true);
}

fn read_file(day: i32, example: bool) -> String {
    let file_path: String;
    if example {
        file_path = format!("inputs/day_{}_example.txt", day);
    } else {
        file_path = format!("inputs/day_{}.txt", day);
    }
    let input: String = std::fs::read_to_string(file_path).expect("Couldn't read file!");
    return input;
}

fn solve_days(example: bool) {
    {
        // Day 1
        const TOP_N_ELVES: i32 = 3;
        let input: String = read_file(1, example);
        days::day_1::part_1(&input);
        days::day_1::part_2(&input, TOP_N_ELVES);
    }
}
