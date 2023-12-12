fn read_input(input: &str, reverse: bool) -> Vec<Vec<i128>> {
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line: &&str| line.len() > 0)
        .collect();

    let mut readings: Vec<Vec<i128>> = Vec::new();

    for line in lines {
        let mut parts: Vec<i128> = line.split(' ')
            .map(|part| part.trim())
            .filter(|part: &&str| part.len() > 0)
            .map(|part| part.parse().unwrap())
            .collect();

        if reverse { parts.reverse(); }
        readings.push(parts);
    }

    return readings;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<(), String> {
        let input = read_aoc_example();

        let readings: Vec<Vec<i128>> = read_input(&input, false);

        println!("{:?}", readings);

        Ok(())
    }

    #[test]
    fn aoc09_part12() -> Result<(), String> {
        let input = read_aoc_challenge();
        let mut readings: Vec<Vec<i128>> = read_input(&input, true);

        for reading in readings.iter_mut() {
            let get_unique_numbers_count = |numbers: &Vec<i128>| {
                let mut unqiue_reading_values: HashSet<i128> = HashSet::new();
                unqiue_reading_values.extend(numbers);
                unqiue_reading_values.len()
            };

            let mut calculations = vec![reading.clone()];

            while get_unique_numbers_count(calculations.last().unwrap()) != 1 {
                let last_calc = calculations.last().unwrap();
                let mut new_calc: Vec<i128> = Vec::new();

                for i in 0..last_calc.len() - 1 {
                    let a = last_calc[i];
                    let b = last_calc[i + 1];

                    new_calc.push(b-a);
                }

                calculations.push(new_calc);
            }

            for j in (0..calculations.len()).rev() {
                let diff: i128 = if j + 1 < calculations.len() {
                    *calculations[j+1usize].last().unwrap()
                } else {
                    0
                };

                let calculation = &mut calculations[j];
                println!("{:?}", calculation);

                let last_number = calculation.last().unwrap();
                calculation.push(last_number + diff);
            }

            let new_reading = *calculations.first().unwrap().last().unwrap();
            reading.push(new_reading);
        }

        println!("Final: {:?}", readings);

        let solution: i128 = readings.iter()
            .map(|reading| reading.last().unwrap())
            .sum();

        println!("Solution: {}", solution);

        Ok(())
    }

    use std::{fs, collections::HashSet};

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc09.txt").expect("Failed to read the input file");
    }

    fn read_aoc_example() -> String {
        return "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        ".to_string();
    }
}
