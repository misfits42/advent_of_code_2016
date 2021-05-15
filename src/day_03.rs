#[aoc_generator(day3)]
fn generate_input(raw_input: &str) -> Vec<(u64, u64, u64)> {
    let mut triangle_candidates: Vec<(u64, u64, u64)> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Split up line and add to candidate list
        let values = line.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        if values.len() != 3 {
            panic!("Day 3 - invalid number of values in line.");
        }
        triangle_candidates.push((values[0], values[1], values[2]));
    }
    return triangle_candidates;
}

#[aoc(day3, part1)]
fn solve_part_1(input: &Vec<(u64, u64, u64)>) -> u64 {
    let mut count = 0;
    for (a, b, c) in input {
        // Check if any of the side combinations indicate an "impossible" triangle
        if check_triangle_validity(*a, *b, *c) {
            count += 1;
        }
    }
    return count;
}

#[aoc(day3, part2)]
fn solve_part_2(input: &Vec<(u64, u64, u64)>) -> u64 {
    // Realign the triangles using the verticle rule
    let mut new_input: Vec<(u64, u64, u64)> = vec![];
    for i in 0..(input.len() / 3) {
        let line_0 = input[3 * i];
        let line_1 = input[3 * i + 1];
        let line_2 = input[3 * i + 2];
        new_input.push((line_0.0, line_1.0, line_2.0));
        new_input.push((line_0.1, line_1.1, line_2.1));
        new_input.push((line_0.2, line_1.2, line_2.2));
    }
    // Now check triangle validity
    let mut count = 0;
    for (a, b, c) in new_input {
        if check_triangle_validity(a, b, c) {
            count += 1;
        }
    }
    return count;
}

/// Checks if the triangle specified by the given sides is valid IAW rules specified in AOC 2016
/// Day 3.
fn check_triangle_validity(a: u64, b: u64, c: u64) -> bool {
    if a + b <= c || b + c <= a || c + a <= b {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d03_p1_proper() {
        let input = generate_input(&read_to_string("./input/2016/day3.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(862, result);
    }

    #[test]
    fn test_d03_p2_proper() {
        let input = generate_input(&read_to_string("./input/2016/day3.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1577, result);
    }
}
