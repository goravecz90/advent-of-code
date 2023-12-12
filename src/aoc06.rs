fn read_input(input: &str) -> Vec<(usize, usize)> {
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| line.len() > 0)
        .collect();

    let times: Vec<usize> = lines.get(0).unwrap()
        .split(':').collect::<Vec<&str>>().get(1).unwrap().trim()
        .split(' ').collect::<Vec<&str>>().iter()
        .filter(|num| num.len() > 0)
        .map(|num_str| num_str.trim().parse().unwrap())
        .collect();

    let distances: Vec<usize> = lines.get(1).unwrap()
        .split(':').collect::<Vec<&str>>().get(1).unwrap().trim()
        .split(' ').collect::<Vec<&str>>().iter()
        .filter(|num| num.len() > 0)
        .map(|num_str| num_str.trim().parse().unwrap())
        .collect();

    return times.into_iter().zip(distances.into_iter()).collect::<Vec<(usize, usize)>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<(), String> {
        let input = read_aoc_example();

        println!("{:?}", read_input(&input));

        Ok(())
    }

    #[test]
    fn aoc06_part1() -> Result<(), String> {
        let input = read_aoc_challenge();
        let races = read_input(&input);

        let options: Vec<usize> = races.iter().map(|&(time, record)| {
            println!("{} {}", time, record);

            return (1..=time).collect::<Vec<usize>>().iter()
                .map(|&i| {
                    let speed = i;
                    let rem_time = time - i;

                    return rem_time * speed;
                })
                .filter(|&distance| distance > record)
                .collect::<Vec<usize>>()
                .len();
        }).collect();

        println!("{:?}", options.iter().product::<usize>());
 
        Ok(())
    }

    use std::fs;

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc06.txt").expect("Failed to read the input file");
    }

    fn read_aoc_example() -> String {
        return "
        Time:      7  15   30
        Distance:  9  40  200
        ".to_string();
    }
}
