
pub fn part1(input: String) -> i32 {
    let mut start = 50;
    let mut c = 0;

    input.as_str().split("\n").for_each(|line| {
        let n = line[1..].parse::<i32>().unwrap();
        let rotation = if line.starts_with('L') {
            -1
        } else {
            1
        };
        start = start + (rotation * (n % 100));
        start = if start > 99 {
            start - 100
        } else if start < 0 {
            start + 100
        } else {
            start
        };

        if start == 0 {
            c = c+1
        }
    });

    c
}

pub fn part2(input: String) -> i32 {
    let mut start = 50;
    let mut c = 0;

    input.as_str().split("\n").for_each(|line| {
        let n = line[1..].parse::<i32>().unwrap();
        let rotation = if line.starts_with('L') {
            -1
        } else {
            1
        };

        let cink = if rotation == 1 {
            (start + n) / 100
        } else {
            (if start == 0 { 0 } else { 100 - start } + n) / 100
        };

        start = start + (rotation * (n % 100));
        start = if start > 99 {
            start - 100
        } else if start < 0 {
            start + 100
        } else {
            start
        };

        c = c+cink;
    });

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.to_string()), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.to_string()), 6);
    }
}
