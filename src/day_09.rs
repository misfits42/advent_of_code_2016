#[aoc_generator(day9)]
fn generate_input(raw_input: &str) -> String {
    // Remove all whitespace from the raw input
    return raw_input.chars().filter(|c| !c.is_whitespace()).collect::<String>();
}

/// Takes input compressed in experimental format (AOC 2016 Day 9) and conduct one pass of
/// decompression. Note that to get the fully decompressed output, multiple decompression passes
/// may be required.
fn decompress_input(input: &String) -> String {
    let mut i = 0;
    let chars = input.chars().collect::<Vec<char>>();
    let mut decompressed = String::new();
    loop {
        // Stop if end of compressed string is reached
        if i >= chars.len() {
            break;
        }
        // Check if current character is beginning of marker
        if chars[i] == '(' {
            i += 1;
            let mut char_window_len = String::new();
            let mut num_repeats = String::new();
            let mut repeats_flag = false;
            loop {
                // Panic if marker window goes past end of compressed string
                if i >= chars.len() {
                    panic!("D9_P1: marker window exceeded end of compressed string.");
                }
                // Check for end of marker window
                if chars[i] == ')' {
                    i += 1;
                    break;
                } else if chars[i] == 'x' { // Check for middle character of window
                    i += 1;
                    repeats_flag = true;
                } else {
                    if repeats_flag {
                        num_repeats.push(chars[i]);
                    } else {
                        char_window_len.push(chars[i]);
                    }
                    i += 1;
                }
            }
            // Try to convert lengths - panic if either are not convertable to usize
            let char_window_len = char_window_len.parse::<usize>().unwrap();
            let num_repeats = num_repeats.parse::<usize>().unwrap();
            let mut repeat_sequence = String::new();
            // Observe the character sequence that is to be repeated
            for _ in 0..char_window_len {
                repeat_sequence.push(chars[i]);
                i += 1;
            }
            // Push the repeat sequence into the decompressed output
            for _ in 0..num_repeats {
                decompressed.push_str(&repeat_sequence);
            }
        } else {
            // Not within a marker or repeat sequence, so character is just added to decompressed
            decompressed.push(chars[i]);
            i += 1;
        }
    }
    return decompressed;
}

#[aoc(day9, part1)]
fn solve_part_1(input: &String) -> usize {
    let decompressed = decompress_input(input);
    return decompressed.len();
}

#[aoc(day9, part2)]
fn solve_part_2(input: &String) -> usize {
    let mut temp_input = input.to_string();
    let mut num_passes = 0;
    loop {
        // Conduct one decompression pass
        let temp_output = decompress_input(&temp_input);
        // Check if the output is the same length as the input
        if temp_output.len() == temp_input.len() {
            return temp_output.len();
        }
        num_passes += 1;
        println!("No. passes: {} ||| Output len: {}", num_passes, temp_output.len());
        temp_input = temp_output;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d09_p1_proper() {
        let input = generate_input(&read_to_string("./input/2016/day9.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(98135, result);
    }
}
