use super::utils::carto::CardinalDirection;
use super::utils::carto::Point2D;
use regex::Regex;
use std::collections::HashSet;

/// Represents the two possible turning directions - left or right.
enum TurnDirection {
    Left,
    Right,
}

#[aoc_generator(day1)]
fn generate_input(raw_input: &str) -> Vec<(TurnDirection, i64)> {
    let mut input: Vec<(TurnDirection, i64)> = vec![];
    let raw_input = raw_input.trim();
    // Create regexes for matching instructions
    let left_regex = Regex::new(r"^L(\d+)$").unwrap();
    let right_regex = Regex::new(r"^R(\d+)$").unwrap();
    for instruction in raw_input.split(", ") {
        if left_regex.is_match(instruction) {
            let captures = left_regex.captures(instruction).unwrap();
            let steps = captures[1].parse::<i64>().unwrap();
            input.push((TurnDirection::Left, steps));
        } else if right_regex.is_match(instruction) {
            let captures = right_regex.captures(instruction).unwrap();
            let steps = captures[1].parse::<i64>().unwrap();
            input.push((TurnDirection::Right, steps));
        } else {
            panic!("Day 1 - invalid instruction in input!");
        }
    }
    return input;
}

/// Updates the current location and direction based on the given turn direction and steps.
fn update_location(
    current_loc: &mut Point2D,
    current_dir: &mut CardinalDirection,
    turn_dir: &TurnDirection,
    steps: &i64,
) {
    // First conduct turn
    match turn_dir {
        TurnDirection::Left => *current_dir = current_dir.rotate_left(),
        TurnDirection::Right => *current_dir = current_dir.rotate_right(),
    }
    // Then take the steps
    match current_dir {
        CardinalDirection::North => current_loc.move_point(0, -steps),
        CardinalDirection::East => current_loc.move_point(*steps, 0),
        CardinalDirection::South => current_loc.move_point(0, *steps),
        CardinalDirection::West => current_loc.move_point(-steps, 0),
    }
}

#[aoc(day1, part1)]
fn solve_part_1(input: &Vec<(TurnDirection, i64)>) -> u64 {
    let start_loc = Point2D::new(0, 0);
    let mut current_loc = start_loc;
    let mut current_dir = CardinalDirection::North;
    for (turn_dir, steps) in input {
        // Execute instruction to update location
        update_location(&mut current_loc, &mut current_dir, turn_dir, steps)
    }
    // Calculate distance from origin
    return current_loc.calculate_manhattan_distance(&start_loc);
}

#[aoc(day1, part2)]
fn solve_part_2(input: &Vec<(TurnDirection, i64)>) -> u64 {
    let start_loc = Point2D::new(0, 0);
    let mut current_loc = start_loc;
    let mut current_dir = CardinalDirection::North;
    let mut visited_locs: HashSet<Point2D> = HashSet::new();
    // Process each instruction until a location is visited twice
    for (turn_dir, steps) in input {
        // First conduct turn
        match turn_dir {
            TurnDirection::Left => current_dir = current_dir.rotate_left(),
            TurnDirection::Right => current_dir = current_dir.rotate_right(),
        }
        // Determine unit vector for updating location
        let unit_vec = match current_dir {
            CardinalDirection::North => (0, -1),
            CardinalDirection::East => (1, 0),
            CardinalDirection::South => (0, 1),
            CardinalDirection::West => (-1, 0),
        };
        // Conduct each step separately and check if each location has been seen
        for _ in 0..*steps {
            current_loc.move_point(unit_vec.0, unit_vec.1);
            if visited_locs.contains(&current_loc) {
                return current_loc.calculate_manhattan_distance(&start_loc);
            }
            // Record each new location
            visited_locs.insert(current_loc);
        }
    }
    // Should have revisited a point - so getting here is an error!
    panic!("D1_P2: did not visit a point twice!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d01_p1_proper() {
        let input = generate_input(&read_to_string("./input/2016/day1.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(332, result);
    }

    #[test]
    fn test_d01_p2_proper() {
        let input = generate_input(&read_to_string("./input/2016/day1.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(166, result);
    }
}
