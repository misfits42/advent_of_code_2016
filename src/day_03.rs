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
        if a + b <= *c || b + c <= *a || c + a <= *b {
            continue;
        }
        count += 1;
    }
    return count;
}

#[aoc(day3, part2)]
fn solve_part_2(_input: &Vec<(u64, u64, u64)>) -> u64 {
    unimplemented!();
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
}
