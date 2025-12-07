const BEAM: u8 = b'|';
const SPACE: u8 = b'.';
const SPLIT: u8 = b'^';
const START: u8 = b'S';

pub fn part1(input: String) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let line_size = lines[0].len();
    let mut last_processed: Vec<u8> = lines[0].as_bytes().to_vec();
    let mut count: i64 = 0;

    for line in lines.iter().skip(1) {
        let line_bytes = line.as_bytes();
        println!("{}", line);
        // Copy the current line
        let mut current_processed: Vec<u8> = line_bytes.to_vec();

        // First pass: propagate beams down
        for i in 0..line_size {
            current_processed[i] = match (current_processed[i], last_processed[i]) {
                (SPACE, BEAM) => BEAM,
                (SPACE, START) => BEAM,
                _ => current_processed[i],
            }
        }

        // Print current and last as strings
        println!("last:    {}", String::from_utf8_lossy(&last_processed));
        println!("current: {}", String::from_utf8_lossy(&current_processed));

        // Second pass: handle splits
        for i in 0..line_size {
            if current_processed[i] == SPLIT && last_processed[i] == BEAM {
                if i > 0 {
                    current_processed[i-1] = BEAM;
                }
                if i+1 < line_size {
                    current_processed[i+1] = BEAM;
                }
                count += 1;

                // FUUuuuuuuuuuuucucuckckck
                // let was_space_left = if i > 0 {
                //     let c = current_processed[i-1];
                //     current_processed[i-1] = BEAM;
                //     c == SPACE
                // } else { false };
                // let was_space_right = if i+1 < line_size {
                //     let c = current_processed[i+1];
                //     current_processed[i+1] = BEAM;
                //     c == SPACE
                // } else { false };
                // println!("{} {} {}", i, was_space_left, was_space_right);
                // if was_space_left || was_space_right {
                //     count += 1;
                // }
            }
        }
        println!("current: {}", String::from_utf8_lossy(&current_processed));

        println!();
        last_processed = current_processed;
    }

    count
}

pub fn part2(input: String) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return 0;
    }

    let line_size = lines[0].len();
    let first_line = lines[0].as_bytes();

    let mut last_processed: Vec<u64> = first_line.iter().map(|&cell| {
        if cell == START { 1 } else { 0 }
    }).collect();

    for line in lines.iter().skip(1) {
        let line_bytes = line.as_bytes();
        let mut current_processed: Vec<u64> = vec![0; line_size];

        for i in 0..line_size {
            if line_bytes[i] == SPACE && last_processed[i] > 0 {
                current_processed[i] = last_processed[i];
            }
        }

        // Second pass: handle splits
        for i in 0..line_size {
            if line_bytes[i] == SPLIT && last_processed[i] > 0 {
                let incoming_timelines = last_processed[i];
                if i > 0 && line_bytes[i-1] == SPACE {
                    current_processed[i-1] += incoming_timelines;
                }
                if i+1 < line_size && line_bytes[i+1] == SPACE {
                    current_processed[i+1] += incoming_timelines;
                }
            }
        }

        last_processed = current_processed;
    }

    last_processed.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.to_string()), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.to_string()), 40);
    }
}
