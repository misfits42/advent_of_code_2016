use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

/// Represents room data, including its encrypted name, sector ID and listed checksum.
struct RoomData {
    encrypted_name: String,
    sector_id: u64,
    checksum: String
}

impl RoomData {
    pub fn new(encrypted_name: String, sector_id: u64, checksum: String) -> Self {
        Self {
            encrypted_name: encrypted_name,
            sector_id: sector_id,
            checksum: checksum
        }
    }

    #[allow(dead_code)]
    pub fn get_encrypted_name(&self) -> String {
        return self.encrypted_name.to_string();
    }

    pub fn get_sector_id(&self) -> u64 {
        return self.sector_id;
    }

    #[allow(dead_code)]
    pub fn get_checksum(&self) -> String {
        return self.checksum.to_string();
    }

    /// Calculates the checksum of room from its encrypted name.
    pub fn calculate_checksum(&self) -> String {
        let encrypted_name_chars = self.encrypted_name.replace("-", "");
        let mut char_counts: HashMap<char, u64> = HashMap::new();
        for c in encrypted_name_chars.chars() {
            if char_counts.contains_key(&c) {
                *char_counts.get_mut(&c).unwrap() += 1;
            } else {
                char_counts.insert(c, 1);
            }
        }
        // Map counts to characters with those counts
        let mut counts: HashMap<u64, Vec<char>> = HashMap::new();
        for (k, v) in char_counts.iter() {
            if counts.contains_key(&v) {
                counts.get_mut(&v).unwrap().push(*k);
            } else {
                counts.insert(*v, vec![*k]);
            }
        }
        // Determines top five most common characters from encrypted name, ties broken alphabetical
        let mut calculated_checksum = String::new();
        for (_count, chars) in counts.into_iter().sorted().rev() {
            let chars = chars.into_iter().sorted().collect::<Vec<char>>();
            for c in chars {
                calculated_checksum.push(c);
                if calculated_checksum.len() == 5 {
                    break;
                }
            }
            if calculated_checksum.len() == 5 {
                break;
            }
        }
        return calculated_checksum;
    }

    /// Calculates checksum from encrypted name and checks if it matches the original checksum.
    pub fn validate_checksum(&self) -> bool {
        let calculated_checksum = self.calculate_checksum();
        return calculated_checksum == self.checksum;
    }
}

#[aoc_generator(day4)]
fn generate_input(raw_input: &str) -> Vec<RoomData> {
    let room_data_regex = Regex::new(r"^(.*)-(\d+)\[(.*)\]$").unwrap();
    let mut rooms: Vec<RoomData> = vec![];
    for line in raw_input.lines() {
        // Trim leading and trailing whitespace, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // All input lines should be correctly formatted
        if !room_data_regex.is_match(line) {
            panic!("Day 4 - incorrect line format in raw input.");
        }
        // Extract room data fields from input line
        let captures = room_data_regex.captures(line).unwrap();
        let encrypted_name = captures[1].to_string();
        let sector_id = captures[2].parse::<u64>().unwrap();
        let checksum = captures[3].to_string();
        let room_data = RoomData::new(encrypted_name, sector_id, checksum);
        rooms.push(room_data);
    }
    return rooms;
}

#[aoc(day4, part1)]
fn solve_part_1(input: &Vec<RoomData>) -> u64 {
    let mut count = 0;
    for room in input {
        if room.validate_checksum() {
            count += room.get_sector_id();
        }
    }
    return count;
}

#[aoc(day4, part2)]
fn solve_part_2(input: &Vec<RoomData>) -> u64 {
    unimplemented!();
}
