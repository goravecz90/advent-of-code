use std::collections::{HashMap, HashSet};

use colored::Colorize;

#[derive(Clone, Debug)]
struct Item {
    value: char,

    is_symbol: bool,
    is_number: bool,
    
    is_engine_part: bool,
}

impl Default for Item {
    fn default() -> Item {
        Item {
            value: 'x',

            is_symbol: false,
            is_number: false,

            is_engine_part: false,
        }
    }
}

fn show_engine(engine: &Vec<Vec<Item>>) {
    for row in engine {
        for item in row {
            if item.is_number && item.is_engine_part {
                print!("{} ", item.value.to_string().bold().green());
            } else if item.is_number {
                print!("{} ", item.value.to_string().bold().yellow());
            } else if item.is_symbol {
                print!("{} ", item.value.to_string().bold().red());
            } else {
                print!("{} ", item.value.to_string().bold());
            }
        }
        println!(); // Newline after each row
    }
}

fn read_input(input: String) -> Vec<Vec<Item>> {
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| line.len() > 0)
        .collect();

    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    let mut engine_map = vec![Vec::new(); height];

    for (i, line) in lines.iter().enumerate() {
        let mut items: Vec<Item> = vec![Item { ..Default::default() }; width];

        if line.len() != width {
            panic!("Expected all lines to be the same width!")
        }

        for (i, c) in line.char_indices() {
            items[i] = Item {
                value: c,
                
                is_number: c.is_numeric(),
                is_symbol: !c.is_numeric() && c != '.',

                is_engine_part: false,
            }
        }

        engine_map[i] = items;
    }

    return engine_map;
}

fn mark_engine_parts(engine: &mut Vec<Vec<Item>>) {
    let max_iter = 100;
    let height: i32 = engine.len().try_into().unwrap();
    let width: i32 = engine.get(0).unwrap().len().try_into().unwrap();

    let mut updated = true;
    let mut iter = 0;

    while iter < max_iter && updated {
        iter += 1;

        let mut mark_as_symbol: Vec<(usize, usize)> = Vec::new();

        for (ui, items) in engine.iter().enumerate() {
            let i: i32 = ui.try_into().unwrap();

            for (uj, item) in items.iter().enumerate() {
                let j: i32 = uj.try_into().unwrap();

                let has_symbol_neighbour = vec![
                    (i - 1, j - 1), (i - 1, j), (i - 1, j + 1),
                    (i, j - 1), (i, j + 1),
                    (i + 1, j - 1), (i + 1, j), (i + 1, j + 1),
                ]
                    .into_iter()
                    .filter(|&(i, j)| i >= 0 && j >= 0 && i < height && j < width)
                    .map(|(i, j)| engine.get(i as usize).unwrap().get(j as usize).unwrap())
                    .any(|neighbour_item| neighbour_item.is_symbol || neighbour_item.is_engine_part);

                if item.is_number && has_symbol_neighbour && !item.is_engine_part {
                    mark_as_symbol.push((ui, uj));
                }
            }
        }

        updated = mark_as_symbol.len() > 0;

        for &(i, j) in mark_as_symbol.iter() {
            let item: &mut Item = engine.get_mut(i).unwrap().get_mut(j).unwrap();
            item.is_engine_part = true;
        }
    }
}

fn get_engine_part_numbers(engine: Vec<Vec<Item>>) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let mut number_string: String = "".to_owned();

    for row in engine.iter() {
        for item in row.iter() {

            if item.is_engine_part && item.is_number {
                number_string.push(item.value);
            } else {
                if number_string.len() > 0 {
                    let number: i32 = number_string.parse().unwrap();
                    numbers.push(number);
                    number_string = "".to_string();
                }
            }
        }
    }

    return numbers;
}

fn get_numbers_to_gears(engine: Vec<Vec<Item>>) -> HashMap<(i32, i32), Vec<i32>> {
    let height: i32 = engine.len().try_into().unwrap();
    let width: i32 = engine.get(0).unwrap().len().try_into().unwrap();

    let mut gears_to_numbers: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    
    let mut current_gears: HashSet<(i32, i32)> = HashSet::new();
    let mut current_number_string: String = "".to_owned();

    for (ui, row) in engine.iter().enumerate() {
        let i: i32 = ui.try_into().unwrap();

        for (uj, item) in row.iter().enumerate() {
            let j: i32 = uj.try_into().unwrap();

            if item.is_engine_part && item.is_number {
                current_number_string.push(item.value);

                let gear_neighbours: Vec<(i32, i32)> = vec![
                    (i - 1, j - 1), (i - 1, j), (i - 1, j + 1),
                    (i, j - 1), (i, j + 1),
                    (i + 1, j - 1), (i + 1, j), (i + 1, j + 1),
                ]
                    .into_iter()
                    .filter(|&(i, j)| i >= 0 && j >= 0 && i < height && j < width)
                    .map(|(i, j)| (
                        (i, j),
                        engine.get(i as usize).unwrap().get(j as usize).unwrap().value
                    ))
                    .filter(|&(_, value)| value == '*')
                    .map(|(coord, _)| coord)
                    .collect();

                    current_gears.extend(gear_neighbours)
            } else {
                if current_number_string.len() > 0 {
                    let number: i32 = current_number_string.parse().unwrap();

                    current_gears.iter().for_each(|&gear| {
                        if gears_to_numbers.contains_key(&gear) {
                            gears_to_numbers.get_mut(&gear).unwrap().push(number);
                        } else {
                            let mut numbers = Vec::new();
                            numbers.push(number);
                            gears_to_numbers.insert(gear, numbers);
                        }
                    });

                    current_number_string = "".to_string();
                    current_gears = HashSet::new();
                }
            }
        }
    }

    return gears_to_numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc03.txt").expect("Failed to read the input file");
    }

    #[test]
    fn read_input_test() -> Result<(), String> {
        let input = "
            467..114..
            ...*......
            ..35..633.  
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";

        let mut engine: Vec<Vec<Item>> = read_input(input.to_owned());
        mark_engine_parts(&mut engine);

        show_engine(&engine);

        let numbers: Vec<i32> = get_engine_part_numbers(engine);
        println!("{:?}", numbers);

        Ok(())
    }

    #[test]
    fn aoc02_part1_test() -> Result<(), String> {
        let input = read_aoc_challenge();

        let mut engine: Vec<Vec<Item>> = read_input(input.to_owned());
        mark_engine_parts(&mut engine);
        let numbers: Vec<i32> = get_engine_part_numbers(engine);
        let sum: i32 = numbers.iter().sum();

        println!("{}", sum);

        Ok(())
    }

    #[test]
    fn aoc02_part2_test() -> Result<(), String> {
        let input = read_aoc_challenge();

        let mut engine: Vec<Vec<Item>> = read_input(input.to_owned());
        mark_engine_parts(&mut engine);
        let ntog: HashMap<(i32, i32), Vec<i32>> = get_numbers_to_gears(engine);

        let sum: i32 = ntog.iter()
            .filter(|&(_, numbers)| numbers.len() == 2)
            .map(|(_, numbers)| numbers.iter().fold(1, |acc, &x| acc * x))
            .sum();

        println!("{}", sum);

        Ok(())
    }
}
