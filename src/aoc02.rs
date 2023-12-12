use std::cmp::max;

#[derive(Eq, PartialEq, Debug)]
struct Draw {
    green: usize,
    red: usize,
    blue: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct Game {
    index: usize,
    draws: Vec<Draw>,
}

fn read_game_line(line: &str) -> Game {
    let parts: Vec<&str> = line.split(':').collect();

    let game_number_string = parts.get(0).unwrap().replace("Game ", "");
    let game_number: usize = game_number_string.parse().unwrap();

    let draw_strings: Vec<&str> = parts.get(1).unwrap().trim().split(';').collect();
    let draws: Vec<Draw> = draw_strings.iter().map(|&draw_string| {
        return draw_string.split(',').fold(
            Draw { red: 0, green: 0, blue: 0 },
            |acc, x| {
                let parts: Vec<&str> = x.trim().split(' ').collect();
                let color: &str = parts.get(1).unwrap();
                let count: usize = parts.get(0).unwrap().parse().unwrap();

                return match color {
                    "red" => Draw {
                        red: acc.red + count,
                        ..acc
                    },
                    "green" => Draw {
                        green: acc.green + count,
                        ..acc
                    },
                    "blue" => Draw {
                        blue: acc.blue + count,
                        ..acc
                    },
                    _ => panic!("Oh no!"),
                };
            })
    }).collect();

    return Game {
        index: game_number,
        draws
    }
}

fn aoc02_part1(input: String, max_red: usize, max_green: usize, max_blue: usize) -> usize {
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let mut sum: usize = 0;

    for line in lines {
        let game = read_game_line(line.as_str());

        let is_invalid = game.draws.iter().any(|draw| {
            draw.red > max_red || draw.blue > max_blue || draw.green > max_green
        });

        if !is_invalid { sum += game.index }
    }

    return sum;
}

fn aoc02_part2(input: String) -> usize {
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let mut sum: usize = 0;

    for line in lines {
        let game = read_game_line(line.as_str());

        let all_draw = game.draws.iter().fold(
            Draw { red: 0, green: 0, blue: 0 },
            |acc, x| {
                return Draw {
                    red: max(acc.red, x.red),
                    green: max(acc.green, x.green),
                    blue: max(acc.blue, x.blue),
                };
            }
        );

        sum += all_draw.red * all_draw.green * all_draw.blue;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc02.txt").expect("Failed to read the input file");
    }

    #[test]
    fn misc() -> Result<(), String> {
        let game = Game {
            index: 2,
            draws: vec![
                Draw { red: 1, green: 2, blue: 0 },
                Draw { red: 1, green: 3, blue: 4 },
                Draw { red: 0, green: 1, blue: 1 },
            ]
        };

        println!("{:?}", game);

        Ok(())
    }

    #[test]
    fn aoc02_part2_test() -> Result<(), String> {
        let input = read_aoc_challenge();

        assert_eq!(aoc02_part2(input), 86036);

        Ok(())
    }

    #[test]
    fn aoc02_part1_test() -> Result<(), String> {
        let input = read_aoc_challenge();

        assert_eq!(aoc02_part1(input, 12, 13, 14), 2600);

        Ok(())
    }

    #[test]
    fn read_game_line_test() -> Result<(), String> {
        let test_string = String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");

        assert_eq!(read_game_line(test_string.as_str()), Game {
            index: 2,
            draws: vec![
                Draw { red: 0, green: 2, blue: 1 },
                Draw { red: 1, green: 3, blue: 4 },
                Draw { red: 0, green: 1, blue: 1 },
            ]
        });

        Ok(())
    }
}