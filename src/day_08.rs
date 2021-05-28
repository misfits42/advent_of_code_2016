use regex::Regex;

enum RotateType {
    Row,
    Column,
}

impl RotateType {
    pub fn from_string(input: &str) -> Option<RotateType> {
        match input {
            "row" => return Some(RotateType::Row),
            "column" => return Some(RotateType::Column),
            _ => return None,
        }
    }
}

enum Instruction {
    Rect {
        x: usize,
        y: usize,
    },
    Rotate {
        rot_type: RotateType,
        vec_num: usize,
        amount: usize,
    },
}

#[aoc_generator(day8)]
fn generate_input(raw_input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    // Regex for matching the different instructions
    let rect_regex = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    let rotate_regex = Regex::new(r"^rotate (row|column) (x|y)=(\d+) by (\d+)$").unwrap();
    for line in raw_input.lines() {
        // Trim whitespace from lines and ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Check for regex match
        if rect_regex.is_match(line) {
            let captures = rect_regex.captures(line).unwrap();
            let x = captures[1].parse::<usize>().unwrap();
            let y = captures[2].parse::<usize>().unwrap();
            instructions.push(Instruction::Rect { x, y });
        } else if rotate_regex.is_match(line) {
            let captures = rotate_regex.captures(line).unwrap();
            // Regex ensures only valid rotation types are matched
            let rot_type = RotateType::from_string(&captures[1]).unwrap();
            match rot_type {
                RotateType::Row => {
                    if &captures[2] != "y" {
                        panic!("Day 8 - incorrect co-ordinate against row rotate type!");
                    }
                }
                RotateType::Column => {
                    if &captures[2] != "x" {
                        panic!("Day 8 - incorrect co-ordinate against column rotate type!");
                    }
                }
            };
            let vec_num = captures[3].parse::<usize>().unwrap();
            let amount = captures[4].parse::<usize>().unwrap();
            instructions.push(Instruction::Rotate {
                rot_type,
                vec_num,
                amount,
            });
        } else {
            panic!("Day 8 - bad input line format!");
        }
    }
    return instructions;
}

fn execute_instructions(
    instructions: &Vec<Instruction>,
    screen_width: usize,
    screen_height: usize,
) -> Vec<Vec<bool>> {
    // Initialise empty screen array - true is on, false is off
    let mut screen_array = vec![vec![false; screen_width]; screen_height];
    for instruct in instructions {
        match instruct {
            Instruction::Rect { x, y } => {
                for row in 0..*y {
                    for col in 0..*x {
                        screen_array[row][col] = true;
                    }
                }
            }
            Instruction::Rotate {
                rot_type,
                vec_num,
                amount,
            } => {
                for _ in 0..*amount {
                    match rot_type {
                        RotateType::Row => {
                            let mut temp = screen_array[*vec_num][0];
                            for i in 0..screen_width {
                                let cursor = temp;
                                temp = screen_array[*vec_num][(i + 1) % screen_width];
                                screen_array[*vec_num][(i + 1) % screen_width] = cursor;
                            }
                        }
                        RotateType::Column => {
                            let mut temp = screen_array[0][*vec_num];
                            for i in 0..screen_height {
                                let cursor = temp;
                                temp = screen_array[(i + 1) % screen_height][*vec_num];
                                screen_array[(i + 1) % screen_height][*vec_num] = cursor;
                            }
                        }
                    }
                }
            }
        }
    }
    return screen_array;
}

#[aoc(day8, part1)]
fn solve_part_1(instructions: &Vec<Instruction>) -> usize {
    let screen_width = 50;
    let screen_height = 6;
    let screen_array = execute_instructions(instructions, screen_width, screen_height);
    return screen_array
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&x| *x == true)
        .count();
}

#[aoc(day8, part2)]
fn solve_part_2(instructions: &Vec<Instruction>) -> String {
    let screen_width = 50;
    let screen_height = 6;
    let screen_array = execute_instructions(instructions, screen_width, screen_height);
    let mut output = String::new();
    output.push('\n');
    for y in 0..screen_height {
        for x in 0..screen_width {
            if screen_array[y][x] == true {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d08_p1_proper() {
        let input = generate_input(&read_to_string("./input/2016/day8.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(123, result);
    }

    #[test]
    fn test_d08_p2_proper() {
        let input = generate_input(&read_to_string("./input/2016/day8.txt").unwrap());
        let answer = String::from("\n\
            .##..####.###..#..#.###..####.###....##.###...###.\n\
            #..#.#....#..#.#..#.#..#....#.#..#....#.#..#.#....\n\
            #..#.###..###..#..#.#..#...#..###.....#.#..#.#....\n\
            ####.#....#..#.#..#.###...#...#..#....#.###...##..\n\
            #..#.#....#..#.#..#.#....#....#..#.#..#.#.......#.\n\
            #..#.#....###...##..#....####.###...##..#....###..\n");
        let result = solve_part_2(&input);
        assert_eq!(answer, result);
    }
}
