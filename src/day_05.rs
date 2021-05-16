use std::collections::HashMap;
use md5;

#[aoc_generator(day5)]
fn generate_input(raw_input: &str) -> String {
    return raw_input.trim().to_string();
}

#[aoc(day5, part1)]
fn solve_part_1(door_id: &String) -> String {
    let mut index = 0;
    let mut password = String::new();
    // Keep processing until full eight-character password is determined
    while password.len() < 8 {
        // Keep looping until a password character is found
        loop {
            let hash_target = format!("{}{}", door_id, index.to_string());
            index += 1;
            let md5_digest = md5::compute(hash_target.as_bytes());
            let md5_hash = format!("{:x}", md5_digest);
            if md5_hash.starts_with("00000") {
                password.push_str(&md5_hash[5..6]);
                break;
            }
        }
    }
    return password;
}

#[aoc(day5, part2)]
fn solve_part_2(door_id: &String) -> String {
    let mut index = 0;
    let mut password_chars: HashMap<usize, String> = HashMap::new();
    // Keep processing until full eight-character password is found
    while password_chars.len() < 8 {
        // Keep looping until a password character is found
        loop {
            let hash_target = format!("{}{}", door_id, index);
            index += 1;
            let md5_digest = md5::compute(hash_target.as_bytes());
            let md5_hash = format!("{:x}", md5_digest);
            if md5_hash.starts_with("00000") {
                // Check if the position is valid and not seen yet
                let position = md5_hash[5..6].to_string();
                if position.parse::<usize>().is_ok() {
                    let position = position.parse::<usize>().unwrap();
                    if !password_chars.contains_key(&position) && position < 8 {
                        // Record the new password character
                        let character = md5_hash[6..7].to_string();
                        password_chars.insert(position, character);
                    }
                }
                break;
            }
        }
    }
    // Reconstruct the password from characters at recorded positions
    let mut password = String::new();
    for i in 0..=7 {
        password.push_str(password_chars.get(&i).unwrap());
    }
    return password;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d05_p1_proper() {
        let input = generate_input(&read_to_string("./input/2016/day5.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!("f77a0e6e", result);
    }

    #[test]
    fn test_d05_p2_proper() {
        let input = generate_input(&read_to_string("./input/2016/day5.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!("999828ec", result);
    }
}
