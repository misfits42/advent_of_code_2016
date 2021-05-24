#[aoc_generator(day7)]
fn generate_input(raw_input: &str) -> Vec<Vec<char>> {
    let mut parsed_input: Vec<Vec<char>> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        parsed_input.push(line.chars().collect::<Vec<char>>());
    }
    return parsed_input;
}

/// Checks if the given IP address (AOC 2016 Day 7) supports TLS.
fn check_ip_addr_for_tls_support(ip_addr: &Vec<char>) -> bool {
    let mut i = 0;
    let mut hypernet_depth = 0;
    let mut have_skipped = true;
    let mut abba_found = false;
    loop {
        // End if 4-char window slides off end of ip_addr
        if i + 3 >= ip_addr.len() {
            break;
        }
        // If skipped, check all characters for start or end of hypernet
        let mut skip = false;
        let range = {
            if have_skipped {
                0..4
            } else {
                3..4
            }
        };
        for n in range {
            if ip_addr[i + n] == '[' {
                hypernet_depth += 1;
                i = i + n;
                skip = true;
                break;
            } else if ip_addr[i + n] == ']' {
                // Unmatched hypernet closing bracket means IP ADDR doesn't support TLS
                if hypernet_depth == 0 {
                    return false;
                }
                hypernet_depth -= 1;
                i = i + n;
                skip = true;
                break;
            }
        }
        if skip {
            have_skipped = true;
            i += 1;
            continue;
        }
        // Look for an ABBA sequence
        have_skipped = false;
        // Check outer and inner pairs align
        if ip_addr[i] == ip_addr[i + 3] && ip_addr[i + 1] == ip_addr[i + 2] {
            // Check outer and inner character pairs are different
            if ip_addr[i] != ip_addr[i + 1] && ip_addr[i + 2] != ip_addr[i + 3] {
                // Check if the ABBA is outside of all a hypernet sequence
                if hypernet_depth == 0 {
                    abba_found = true;
                } else {
                    return false;
                }
            }
        }
        // Finished checking current window, so slide right by one character
        i += 1;
    }
    // If there are unmatch hyperet sequence brackets, IP ADDR doesn't support TLS
    if hypernet_depth != 0 {
        return false;
    }
    return abba_found;
}

#[aoc(day7, part1)]
fn solve_part_1(input: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    for ip_addr in input {
        if check_ip_addr_for_tls_support(ip_addr) {
            count += 1;
        }
    }
    return count;
}

#[aoc(day7, part2)]
fn solve_part_2(_input: &Vec<Vec<char>>) -> u64 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d07_p1_proper() {
        let input = generate_input(&read_to_string("./input/2016/day7.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(115, result);
    }
}
