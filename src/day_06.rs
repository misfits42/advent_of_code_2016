use std::collections::HashMap;

#[aoc_generator(day6)]
fn generate_input(raw_input: &str) -> Vec<Vec<char>> {
    let mut messages: Vec<Vec<char>> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        messages.push(line.chars().collect::<Vec<char>>());
    }
    return messages;
}

#[aoc(day6, part1)]
fn solve_part_1(input: &Vec<Vec<char>>) -> String {
    // Initialise the position counts
    let mut position_counts: Vec<HashMap<char, u64>> = vec![];
    for _ in 0..8 {
        position_counts.push(HashMap::new());
    }
    // Determine count for each letter in each character index
    for message in input {
        for i in 0..8 {
            if position_counts[i].contains_key(&message[i]) {
                *position_counts[i].get_mut(&message[i]).unwrap() += 1;
            } else {
                position_counts[i].insert(message[i], 1);
            }
        }
    }
    // Determine most frequent character for each position
    let mut decoded_message = String::new();
    for i in 0..8 {
        let top_char = *position_counts[i].iter().max_by_key(|entry| entry.1).unwrap().0;
        decoded_message.push(top_char);
    }
    return decoded_message;
}

#[aoc(day6, part2)]
fn solve_part_2(input: &Vec<Vec<char>>) -> String {
    // Initialise the position counts
    let mut position_counts: Vec<HashMap<char, u64>> = vec![];
    for _ in 0..8 {
        position_counts.push(HashMap::new());
    }
    // Determine count for each letter in each character index
    for message in input {
        for i in 0..8 {
            if position_counts[i].contains_key(&message[i]) {
                *position_counts[i].get_mut(&message[i]).unwrap() += 1;
            } else {
                position_counts[i].insert(message[i], 1);
            }
        }
    }
    // Determine least frequent character for each position
    let mut decoded_message = String::new();
    for i in 0..8 {
        let bot_char = *position_counts[i].iter().min_by_key(|entry| entry.1).unwrap().0;
        decoded_message.push(bot_char);
    }
    return decoded_message;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d06_p1_proper() {
        let input = generate_input(&read_to_string("./input/2016/day6.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!("dzqckwsd", result);
    }

    #[test]
    fn test_d06_p2_proper() {
        let input = generate_input(&read_to_string("./input/2016/day6.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!("lragovly", result);
    }
}
