use std::io::BufRead;

pub fn get(input: &[u8], width: i64, height: i64, seq_pos: i64, x_offset: i8, y_offset: i8) -> u64 { // 1 = paper
    let pos_x: i64 = (seq_pos % width) + x_offset as i64;
    let pos_y: i64 = (seq_pos / width) + y_offset as i64;
    let seq_pos: i64 = (pos_y * width) + pos_x;

    if (pos_x < 0) || (pos_y < 0) { return 0 }
    if (pos_x >= width) || (pos_y >= height) { return 0 }

    if input[seq_pos as usize] == b'@' { 1 } else { 0 }
}

pub fn part1(input: String) -> i32 {
    let copy = input.replace("\n", "");
    let bytes = copy.as_bytes();
    let lines: Vec<&str> = input.split("\n").collect();
    let height: i64 = lines.len() as i64;
    let width: i64 = lines[0].len() as i64;

    let mut count = 0;

    for i in 0..bytes.len() {
        let sum = get(&bytes, width, height, i as i64, -1, -1)
                + get(&bytes, width, height, i as i64, 0, -1)
                + get(&bytes, width, height, i as i64, 1, -1)
                + get(&bytes, width, height, i as i64, -1, 0)
                + get(&bytes, width, height, i as i64, 1, 0)
                + get(&bytes, width, height, i as i64, -1, 1)
                + get(&bytes, width, height, i as i64, 0, 1)
                + get(&bytes, width, height, i as i64, 1, 1);

        let is_paper = bytes[i as usize] == b'@';

        if sum < 4 && is_paper {
            count = count + 1;
            print!("x");
        } else {
            print!("{}", char::from(bytes[i as usize]));
        }
        if (i + 1) % width as usize == 0 {
            println!();
        }
    }

    count
}


pub fn alg(bytes: &[u8], width: i64, height: i64) -> i32 {
    let mut mut_bytes = bytes.to_vec();
    let mut count = 0;

    for i in 0..bytes.len() {
        let sum = get(&bytes, width, height, i as i64, -1, -1)
            + get(&bytes, width, height, i as i64, 0, -1)
            + get(&bytes, width, height, i as i64, 1, -1)
            + get(&bytes, width, height, i as i64, -1, 0)
            + get(&bytes, width, height, i as i64, 1, 0)
            + get(&bytes, width, height, i as i64, -1, 1)
            + get(&bytes, width, height, i as i64, 0, 1)
            + get(&bytes, width, height, i as i64, 1, 1);

        let is_paper = bytes[i as usize] == b'@';

        if sum < 4 && is_paper {
            count = count + 1;
            print!("x");
            mut_bytes[i] = b'.';
        } else {
            print!("{}", char::from(bytes[i as usize]));
        }
        if (i + 1) % width as usize == 0 {
            println!();
        }
    }
    println!();
    println!();

    if count == 0 {
        0
    } else {
        count + alg(&mut_bytes, width, height)
    }
}

pub fn part2(input: String) -> i32 {
    let copy = input.replace("\n", "");
    let bytes = copy.as_bytes();
    let lines: Vec<&str> = input.split("\n").collect();
    let height: i64 = lines.len() as i64;
    let width: i64 = lines[0].len() as i64;

    return alg(&bytes, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE.to_string()), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE.to_string()), 43);
    }
}
