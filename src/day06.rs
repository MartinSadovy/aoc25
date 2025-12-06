pub fn part1(input: String) -> i64 {
    let boxes: Vec<Vec<&str>> = input.as_str().split("\n").map(|a|
        a.trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let ops = boxes.last().unwrap();
    let mut res: Vec<i64> = vec![0; ops.len()];

    for line in 0..boxes.len()-1 { // except last line
        for j in 0..boxes[line].len() {
            let v = boxes[line][j].parse::<i64>().unwrap();
            if line == 0 {
                res[j] = v;
            } else {
                res[j] = match ops[j] {
                    "+" => res[j] + v,
                    "*" => res[j] * v,
                    _ => panic!("Unknown op {}", ops[j])
                }
            }

        }
    }

    res.into_iter().sum::<i64>()
}

pub fn part2(input: String) -> i64 {
    let lines = input.as_str().split("\n").collect::<Vec<_>>();
    let lines_width = lines[0].len();
    let lines_count = lines.len();
    let lines_bytes: Vec<&[u8]> = lines.iter().map(|s| s.as_bytes()).collect();

    let mut result: i64 = 0;

    let mut vbuf: Vec<String> = vec![String::new(); 10];
    let mut vbuf_i: usize = 0;

    let mut index = lines_width-1;
    while index >= 0 {
        for j in 0..lines_count-1 { // except last line
            vbuf[vbuf_i].push(lines_bytes[j][index] as char);
        }
        let op_c = lines_bytes[lines_count-1][index] as char;
        if op_c != ' ' {
            let is_multiply = match op_c {
                '+' => false,
                '*' => true,
                _ => panic!("Unknown op {}", op_c)
            };
            let mut collector = if is_multiply { 1 } else { 0 };
            for s in &vbuf {
                if s.trim().is_empty() {
                    continue;
                }
                let a = s.trim().parse::<i64>().unwrap_or(1);
                collector = if is_multiply { collector * a } else { collector + a };
            }

            result = result + collector;

            vbuf = vec![String::new(); 10];
            vbuf_i = 0;

            if index < 2 {
                break;
            }
            index = index - 2; // empty column
        } else {
            vbuf_i = vbuf_i + 1;

            if index < 1 {
                break;
            }
            index = index - 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.to_string()), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.to_string()), 3263827);
    }
}
