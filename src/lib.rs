pub mod day01;
pub mod day02;


pub fn read_input(day: u8) -> String {
    std::fs::read_to_string(format!("inputs/day{:02}.txt", day))
        .unwrap_or_else(|_| panic!("Failed to read input for day {}", day))
}
