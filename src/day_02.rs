use super::utils::carto::Point2D;

/// Represents the possible movement directions used to determine keypad presses.
enum MoveDir {
    Up,
    Down,
    Left,
    Right
}

impl MoveDir {
    /// Determines the unit vector corresponding to the movement direction.
    fn get_unit_vector(&self) -> (i64, i64) {
        match self {
            MoveDir::Up => (0, -1),
            MoveDir::Down => (0, 1),
            MoveDir::Left => (-1, 0),
            MoveDir::Right => (1, 0)
        }
    }
}

#[aoc_generator(day2)]
fn generate_input(raw_input: &str) -> Vec<Vec<MoveDir>> {
    let mut button_moves: Vec<Vec<MoveDir>> = vec![];
    for line in raw_input.lines() {
        // Trim line and ignore acursor_loc: &Point2Dny empty lines
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        // Parse current line to determine movement directions
        let mut single_button: Vec<MoveDir> = vec![];
        for c in line.chars() {
            match c {
                'U' => single_button.push(MoveDir::Up),
                'D' => single_button.push(MoveDir::Down),
                'L' => single_button.push(MoveDir::Left),
                'R' => single_button.push(MoveDir::Right),
                _ => panic!("D3: bad character in raw input!"),
            }
        }
        button_moves.push(single_button);
    }
    return button_moves;
}

/// Adjusts the given location to ensure it stays within the 3x3 square centred on (1, 1).
fn adjust_cursor_location_bounds(cursor_loc: &Point2D) -> Point2D {
    let mut adjusted_loc = *cursor_loc;
    // Adjust x co-ord
    if adjusted_loc.get_x() < 0 {
        adjusted_loc.set_x(0);
    } else if adjusted_loc.get_x() > 2 {
        adjusted_loc.set_x(2);
    }
    // Adjust y co-ord
    if adjusted_loc.get_y() < 0 {
        adjusted_loc.set_y(0);
    } else if adjusted_loc.get_y() > 2 {
        adjusted_loc.set_y(2);
    }
    return adjusted_loc;
}

/// Calculates the resulting button press from the cursor location.
fn calculate_button_press(cursor_loc: &Point2D) -> String {
    let result = 1 + cursor_loc.get_x() + 3 * cursor_loc.get_y();
    return result.to_string();
}

#[aoc(day2, part1)]
fn solve_part_1(input: &Vec<Vec<MoveDir>>) -> String {
    let mut code = String::new();
    let mut cursor_loc = Point2D::new(1, 1); // Corresponds to "5" key as starting point
    for single_button in input {
        for movement in single_button {
            // Move cursor and adjust to stay within 3x3 square centred on "5" key at (1, 1)
            let unit_vec = movement.get_unit_vector();
            cursor_loc.move_point(unit_vec.0, unit_vec.1);
            cursor_loc = adjust_cursor_location_bounds(&cursor_loc);
        }
        // Determine key from resulting code
        let button_press = calculate_button_press(&cursor_loc);
        code.push_str(&button_press);
    }
    return code;
}

#[aoc(day2, part2)]
fn solve_part_2(_input: &Vec<Vec<MoveDir>>) -> String {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d01_p1_proper() {
        let input = generate_input(&read_to_string("./input/2016/day2.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!("78985", result);
    }
}
