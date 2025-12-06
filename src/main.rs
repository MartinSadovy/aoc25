use advent_of_code::*;

fn main() {

    // let day = std::env::args()
    //     .nth(1)
    //     .and_then(|s| s.parse::<u8>().ok())
    //     .unwrap_or(1);

    let day = 6;
    let input = read_input(day);
    let input2 = input.to_string();
    match day {
        1 => {
            println!("Day 1 Part 1: {}", day01::part1(input));
            println!("Day 1 Part 2: {}", day01::part2(input2));
        }
        2 => {
            println!("Day 2 Part 1: {}", day02::part1(input));
            println!("Day 2 Part 2: {}", day02::part2(input2));
        }
        3 => {
            println!("Day 3 Part 1: {}", day03::part1(input));
            println!("Day 3 Part 2: {}", day03::part2(input2));
        }
        4 => {
            println!("Day 4 Part 1: {}", day04::part1(input));
            println!("Day 4 Part 2: {}", day04::part2(input2));
        }
        5 => {
            println!("Day 5 Part 1: {}", day05::part1(input));
            println!("Day 5 Part 2: {}", day05::part2(input2));
        }
        6 => {
            println!("Day 6 Part 1: {}", day06::part1(input));
            println!("Day 6 Part 2: {}", day06::part2(input2));
        }
        _ => println!("Day {} not implemented yet", day),
    }
}
