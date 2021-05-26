use std::collections::HashSet;

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

/// Checks if the given IP address (AOC 2016 Day 7) supports TLS (transport-layer snooping)
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

/// Checks if the given IP address (AOC 2016 Day 7) supports SSL (super-secret listening).
fn check_ip_addr_for_ssl_support(ip_addr: &Vec<char>) -> bool {
    // Track windows start index and hypernet depth
    let mut i = 0;
    let mut hypernet_depth = 0;
    let mut aba_candidates: HashSet<String> = HashSet::new();
    let mut bab_candidates: HashSet<String> = HashSet::new();
    // Only check all windows characters for hypernet brackets if previous move was a skip
    let mut have_skipped = true;
    loop {
        // End if 3-char window slides off end of ip_addr or invalid combination of hypernet braces
        if i + 2 >= ip_addr.len() || hypernet_depth < 0 {
            break;
        }
        let mut skip = false;
        let range = {
            if have_skipped {
                0..3
            } else {
                2..3
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
        // Look for ABA or BAB depending on if within hypernet sequence or not
        if hypernet_depth == 0 {
            // Outside of any hypernet sequence - add window as an ABA
            if ip_addr[i] == ip_addr[i + 2] {
                let mut candidate = String::new();
                for n in 0..3 {
                    candidate.push(ip_addr[i + n])
                }
                aba_candidates.insert(candidate);
            }
        } else {
            // Within hypernet sequence - check for BAB matching
            if ip_addr[i] == ip_addr[i + 2] {
                // Invert inner and outer characters for comparison against ABA candidates
                let outer = ip_addr[i];
                let inner = ip_addr[i + 1];
                let mut candidate = String::new();
                candidate.push(inner);
                candidate.push(outer);
                candidate.push(inner);
                bab_candidates.insert(candidate);
            }
        }
        // Finished checking current window, so slide right by one character
        i += 1;
    }
    // IP ADDR does not supper SSL if hypernet brackets not correct
    if hypernet_depth < 0 {
        return false
    }
    // Look for overlap between ABA and BAB candidates
    let overlap = aba_candidates.intersection(&bab_candidates).count();
    return overlap >= 1;
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
fn solve_part_2(input: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    for ip_addr in input {
        if check_ip_addr_for_ssl_support(ip_addr) {
            count += 1;
        }
    }
    return count;
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

    #[test]
    fn test_d07_p2_proper() {
        let input = generate_input(&read_to_string("./input/2016/day7.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(231, result);
    }
}
