pub fn part1(input: String) -> i32 {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut count = 0;
    input.as_str().split("\n").for_each(|line| {
        if line.contains("-") {
            let mut nums = line.split("-");
            ranges.push(
                (
                    nums.next().unwrap().parse::<u64>().unwrap(),
                    nums.next().unwrap().parse::<u64>().unwrap()
                )
            );
        } else if line == "" {

        } else {
            let n = line.parse::<u64>().unwrap();

            let any = ranges.iter().filter(|range| (range.0..=range.1).contains(&n)).count() > 0;
            if any {
                println!("{}", n);
                count = count + 1;
            }
        }
    });

    count
}

pub fn part2(input: String) -> usize {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut count: usize = 0;
    input.as_str().split("\n").for_each(|line| {
        if line.contains("-") {
            let mut nums = line.split("-");
            let start = nums.next().unwrap().parse::<u64>().unwrap();
            let end = nums.next().unwrap().parse::<u64>().unwrap();

            ranges.push((start, end));
        }
    });

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut range_n = 0;
    while (range_n < ranges.len()-1) {
        let start = ranges[range_n].0;
        let mut end = ranges[range_n].1;
        for i in range_n+1..ranges.len() {
            if ranges[i].0 > end {
                range_n = i;
                break; // does not intersect with next range
            }
            if end < ranges[i].1 {
                end = ranges[i].1;
            }
            range_n = i;
        }
        count = count + (end - start + 1) as usize;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.to_string()), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.to_string()), 14);
    }
}
