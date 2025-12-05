
pub fn part1(input: String) -> u64 {
    let mut v = 0;
    input.as_str().split(",").for_each(|line| {
        let mut ranges = line.split("-");
        let start: u64 = ranges.next().unwrap().parse::<u64>().unwrap();
        let end: u64 = ranges.next().unwrap().parse::<u64>().unwrap();

        for n in start..=end {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let str = s.as_str();
                let half = (s.len()/ 2) as usize;
                if str[0.. half] == str[half..] {
                    v=v+n;
                }
            }
        }
    });

    v
}

pub fn part2(input: String) -> u64 {
    let mut v = 0;
    input.as_str().split(",").for_each(|line| {
        let mut ranges = line.split("-");
        let start: u64 = ranges.next().unwrap().parse::<u64>().unwrap();
        let end: u64 = ranges.next().unwrap().parse::<u64>().unwrap();

        for n in start..=end {
            let s = n.to_string();
            let mut nyes = false;
            for i in 1..=s.len()/2 {
                if s.len() % i != 0 {
                    continue
                }
                let str = s.as_str();
                let mut yes = true;
                for u in 1..(s.len() / i) {
                    yes = yes && str[0.. i] == str[i*u..i*(u+1)]
                }

                nyes = nyes || yes;
            }
            if nyes {
                v=v+n;
            }
        }
    });
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862";
    const EXAMPLE2: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.to_string()), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2.to_string()), 4174379265);
    }
}
