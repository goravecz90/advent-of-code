fn parse_line(line: &str) -> (Vec<i32>, Vec<i32>) {
    let parts: Vec<&str> = line.split(':').map(|line| line.trim()).collect();
    let winning_numbers_and_bets: &str = parts.get(1).unwrap();
    let winning_numbers_and_bets_parts: Vec<&str> = winning_numbers_and_bets.split('|').collect();

    let winning_numbers: Vec<i32> = winning_numbers_and_bets_parts.get(0).unwrap()
        .split(' ')
        .map(|number_string| number_string.trim())
        .filter(|number_string| number_string.len() > 0)
        .map(|number_string| number_string.parse().unwrap())
        .collect();

    let bets: Vec<i32> = winning_numbers_and_bets_parts.get(1).unwrap()
        .split(' ')
        .map(|number_string| number_string.trim())
        .filter(|number_string| number_string.len() > 0)
        .map(|number_string| number_string.parse().unwrap())
        .collect();

    return (winning_numbers, bets);
}

fn get_line_points(line: &str) -> u64 {
    let (winning_numbers, bets) = parse_line(line);

    let number_of_matches: u32 = winning_numbers.iter()
        .filter(|&&winning_number| bets.contains(&winning_number))
        .count().try_into().unwrap();

    let points =  if number_of_matches == 0 { 0 } else { (2 as u64).pow(number_of_matches - 1) };
    println!("{} - {:?} -- {:?}", points, winning_numbers, bets);
    
    return points;
}

fn get_line_number_of_matches(line: &str) -> u32 {
    let (winning_numbers, bets) = parse_line(line);

    let number_of_matches: u32 = winning_numbers.iter()
        .filter(|&&winning_number| bets.contains(&winning_number))
        .count().try_into().unwrap();
    
    return number_of_matches;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc04.txt").expect("Failed to read the input file");
    }

    fn read_aoc_example() -> String {
        return "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ".to_string();
    }

    #[test]
    fn aoc04_part1() -> Result<(), String> {
        let input = read_aoc_challenge();

        let lines: Vec<String> = input
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| line.len() > 0)
            .collect();

        let sum: u64 = lines.iter().map(|line| get_line_points(line)).sum();
        println!("{}", sum);

        Ok(())
    }

    #[test]
    fn aoc04_part2() -> Result<(), String> {
        let input = read_aoc_challenge();

        let lines: Vec<String> = input
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| line.len() > 0)
            .collect();

        let scratchcard_num = lines.len();
        let mut scratches: Vec<u64> = vec![1; scratchcard_num as usize];

        for (i, line)  in lines.iter().enumerate() {
            let scratch_attempts = scratches.get(i).unwrap().clone(); 
            let matches = get_line_number_of_matches(line) as usize;

            for j in 1 .. matches + 1 {
                let next_index = i + j;
                if next_index < scratchcard_num {
                    scratches[next_index] += scratch_attempts;
                }
            }
        }

        println!("{}", scratches.iter().sum::<u64>());

        Ok(())
    }
}
