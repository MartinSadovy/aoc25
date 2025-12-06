pub fn part1(input: String) -> u32 {
    let mut v = 0;
    input.as_str().split("\n").for_each(|line| {
        let lc = line.chars().last().unwrap();
        let mut largest: char = '0';
        let mut second_largest: char = lc;
        for c in line.chars().take(line.len() - 1) {
            if largest < c {
                largest = c;
                second_largest = lc;

            } else if second_largest < c {
                second_largest = c;
            }
        }
        let n = largest as u32 - '0' as u32;
        let m = second_largest as u32 - '0' as u32;
        let s = (n*10) + m;

        v = v + s;
        println!("{} {} {}", s, n, m);
    });

    v
}

pub fn part2(input: String) -> u64 {
    let mut v = 0;
    input.as_str().split("\n").for_each(|line| {
        let mut largest: [char; 12] = ['0','0','0','0','0','0','0','0','0','0','0','0'];
        for (i, c) in line.chars().enumerate() {
            let mut r = false;
            let min = if i > (line.len() - 12) { 12 - (line.len() - i) } else { 0 };
            for u in min..12 {
                if r {
                    largest[u] = '0';
                } else if largest[u] < c {
                    largest[u] = c;
                    r = true;
                }
            }
        }
        let sum: u64 = largest.iter().rev().enumerate().map(|(i, v)| {
            10u64.pow(i as u32) * (*v as u64 - '0' as u64)
        }).sum();
        v = v + sum;
    });

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE3: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE3.to_string()), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE3.to_string()), 3121910778619);
    }
}
