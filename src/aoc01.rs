fn aoc01_part1(input: String) -> i32{
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut sum = 0;

    for line in lines {
        let reversed_line: String = line.chars().rev().collect();

        let (_, left_number) = get_first_digit(line.as_str()).unwrap();
        let (_, right_number) = get_first_digit(reversed_line.as_str()).unwrap();

        let combined_number_string = {
            let mut temp = "".to_owned();
            temp.push(left_number);
            temp.push(right_number);
            temp
        };

        let number: i32 = combined_number_string.parse().unwrap();
        sum += number;
    }

    return sum;
}

fn aoc01_part2(input: String) -> i32{
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut sum = 0;

    for line in lines {
        println!("Line: {}", line);

        let reversed_line: String = line.chars().rev().collect();

        let (left_digit_index, left_digit) = get_first_digit(line.as_str()).unwrap_or((usize::MAX, 'x'));
        let (left_written_digit_index, left_written_digit) = get_first_written_digit(line.as_str(), false).unwrap_or((usize::MAX, 'x'));
        let left_final_digit = if left_digit_index < left_written_digit_index { left_digit } else { left_written_digit };

        println!("i{}:{} - i{}:{}",
                 left_digit_index,
                 left_digit,
                 left_written_digit_index,
                 left_written_digit
        );

        let (right_digit_index, right_digit) = get_first_digit(reversed_line.as_str())
            .unwrap_or((0, 'x'));
        let (right_written_digit_index, right_written_digit) = get_first_written_digit(line.as_str(), true)
            .unwrap_or((0, 'x'));
        let right_final_digit = if (line.len() - right_digit_index - 1) < right_written_digit_index {
            right_written_digit
        } else {
            right_digit
        };

        println!("i{}:{} - i{}:{}",
                 (line.len() - right_digit_index - 1),
                 right_digit,
                 right_written_digit_index,
                 right_written_digit
        );

        let combined_number_string = {
            let mut temp = "".to_owned();
            temp.push(left_final_digit);
            temp.push(right_final_digit);
            temp
        };

        println!("Number: {}", combined_number_string);

        let number: i32 = combined_number_string.parse().unwrap();
        sum += number;
    }

    return sum;
}

fn get_first_digit(input: &str) -> Option<(usize, char)> {
    for (i, c) in input.char_indices() {
        if c.is_numeric() {
            return Some((i, c));
        }
    }

    return None;
}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref NUMBERS: HashMap<&'static str, char> = {
        let mut map: HashMap<&str, char> = HashMap::new();
        map.insert("one", '1');
        map.insert("two", '2');
        map.insert("three", '3');
        map.insert("four", '4');
        map.insert("five", '5');
        map.insert("six", '6');
        map.insert("seven", '7');
        map.insert("eight", '8');
        map.insert("nine", '9');
        map
    };
}

fn get_first_written_digit(input: &str, reverse: bool) -> Option<(usize, char)> {
    let locations: HashMap<&str, Option<usize>> = NUMBERS
        .iter()
        .map(|(key, _)| (*key, if reverse { input.rfind(key) } else { input.find(key) }))
        .collect();

    let minimum = locations.iter()
        .filter_map(|(key, &value)| Some((*key, value?))) // Filter out None values and dereference key
        .min_by_key(|&(_, value)| value);

    let maximum = locations.iter()
        .filter_map(|(key, &value)| Some((*key, value?))) // Filter out None values and dereference key
        .max_by_key(|&(_, value)| value);

    return if reverse {
        match maximum {
            None => None,
            Some((key, value)) => Some((value, *NUMBERS.get(key).unwrap()))
        }
    } else {
        match minimum {
            None => None,
            Some((key, value)) => Some((value, *NUMBERS.get(key).unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc01.txt").expect("Failed to read the input file");
    }

    #[test]
    fn multi_line_part1() -> Result<(), String> {
        let test_string = String::from("asd800as3443agda200\r\nasd8agda9");

        assert_eq!(aoc01_part1(test_string), 169);

        Ok(())
    }

    #[test]
    fn get_first_written_digit_test() -> Result<(), String> {
        let test_string = String::from("asthree800afours3443agdsixa200");

        assert_eq!(get_first_written_digit(test_string.as_str(), false), Some((2, '3')));
        assert_eq!(get_first_written_digit(test_string.as_str(), true), Some((23, '6')));

        assert_eq!(get_first_digit(test_string.as_str()), Some((7, '8')));

        Ok(())
    }

    #[test]
    fn multi_line_part2() -> Result<(), String> {
        let test_string = String::from("astwod800as3443agda200\r\nasd8agda9eight");

        assert_eq!(aoc01_part2(test_string), 108);

        Ok(())
    }

    #[test]
    fn solve_challenge_part1() -> Result<(), String> {
        let challenge_input = read_aoc_challenge();

        assert_eq!(aoc01_part1(challenge_input), 55971);

        Ok(())
    }

    #[test]
    fn solve_challenge_part2() -> Result<(), String> {
        let challenge_input = read_aoc_challenge();

        assert_eq!(aoc01_part2(challenge_input), 54719);

        Ok(())
    }
}