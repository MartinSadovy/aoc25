use advent_of_code::*;

fn main() {

    // let day = std::env::args()
    //     .nth(1)
    //     .and_then(|s| s.parse::<u8>().ok())
    //     .unwrap_or(1);

    let day = 2;
    let input = read_input(1);
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

        _ => println!("Day {} not implemented yet", day),
    }
}
