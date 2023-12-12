fn read_seeds(input: &str) -> Vec<usize> {
    let lines: Vec<String> = input
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| line.len() > 0)
            .collect();
    
    for line in lines {
        if !line.starts_with("seeds: ") { continue; }

        let numbers: Vec<usize> = line.split(':')
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split(' ')
            .map(|numner_string| numner_string.trim())
            .filter(|number_string| number_string.len() > 0)
            .map(|number_string| number_string.parse().unwrap())
            .collect();

        return numbers;
    }

    panic!("No seed!!!");
}

fn get_map_lines(input: &str, map_name: &str) -> Vec<Vec<usize>> {
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| line.len() > 0)
        .collect();

    let mut is_line_found = false;
    let mut map: Vec<Vec<usize>> = Vec::new();

    for line in lines {
        if !is_line_found {
            if line.starts_with(map_name) { 
                is_line_found = true;
                continue;
            } else { 
                continue;
            }
        }   

        if is_line_found && !line.contains(':') {
            let numbers: Vec<usize> = line
                .split(' ')
                .map(|numner_string| numner_string.trim())
                .filter(|number_string| number_string.len() > 0)
                .map(|number_string| number_string.parse().unwrap())
                .collect();

            map.push(numbers);
        } else {
            map.sort_by(|a, b| {
                let num_a = a.get(1).unwrap();
                let num_b = b.get(1).unwrap();

                return num_a.cmp(num_b);
            });
            return map;
        }        
    }

    if map.len() > 0 {
        map.sort_by(|a, b| {
            let num_a = a.get(1).unwrap();
            let num_b = b.get(1).unwrap();

            return num_a.cmp(num_b);
        });
        return map;
    }

    panic!("Map '{}' not found!!", map_name);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<(), String> {
        let input = read_aoc_example();

        println!("{:?}", read_seeds(&input));

        let map = get_map_lines(&input, "fertilizer-to-water");
        println!("{:?}", map);

        Ok(())
    }

    #[test]
    fn aoc05_part1() -> Result<(), String> {
        let input = read_aoc_challenge();

        let seeds = read_seeds(&input);
        let maps: Vec<Vec<Vec<usize>>> = vec![
            get_map_lines(&input, "seed-to-soil"),
            get_map_lines(&input, "soil-to-fertilizer"),
            get_map_lines(&input, "fertilizer-to-water"),
            get_map_lines(&input, "water-to-light"),
            get_map_lines(&input, "light-to-temperature"),
            get_map_lines(&input, "temperature-to-humidity"),
            get_map_lines(&input, "humidity-to-location"),
        ];

        let mut final_seeds: Vec<usize> = Vec::new();

        for seed in seeds {
            let mut seed_number = seed;

            for map in &maps {
                for line in map {
                    let destination: usize = line.get(0).unwrap().clone();
                    let source: usize = line.get(1).unwrap().clone();
                    let length: usize = line.get(2).unwrap().clone();

                    if source <= seed_number && seed_number < source + length {
                        seed_number = seed_number - source + destination;
                        break;
                    }
                }

            }

            final_seeds.push(seed_number.clone());
        }

        println!("{:?}", final_seeds.iter().min());

        Ok(())
    }

    #[test]
    fn aoc05_part2() -> Result<(), String> {
        let input = read_aoc_challenge();

        let seeds = read_seeds(&input);
        let maps: Vec<Vec<Vec<usize>>> = vec![
            get_map_lines(&input, "seed-to-soil"),
            get_map_lines(&input, "soil-to-fertilizer"),
            get_map_lines(&input, "fertilizer-to-water"),
            get_map_lines(&input, "water-to-light"),
            get_map_lines(&input, "light-to-temperature"),
            get_map_lines(&input, "temperature-to-humidity"),
            get_map_lines(&input, "humidity-to-location"),
        ];

        let mut final_ranges: Vec<(usize, usize)> = Vec::new();
        let mut ranges: Vec<(usize, usize)> = Vec::new();

        for chunk in seeds.chunks(2) {
            match *chunk {
                [seed, length] => {
                    final_ranges.push((seed, length))
                },
                _ => unreachable!(), // This should never happen if you're sure the vector has an even number of elements
            }
        }

        println!("Starting ranges: {:?}", final_ranges);

        for map in &maps {
            ranges.clear();
            ranges.extend(final_ranges.clone());
            final_ranges.clear();

            while ranges.len() > 0 {
                let mut processed = false;
                let (mut first_seed, last_seed) = {
                    let (seed, length) = ranges.pop().unwrap();
                    let last_seed = seed + length - 1;

                    (seed, last_seed)
                };

                println!("Processing seeds from {} to {}", first_seed, last_seed);

                for line in map {
                    let first_destination_seed: usize = line.get(0).unwrap().clone();
                    let (first_source_seed, last_source_seed) = {
                        let seed: usize = line.get(1).unwrap().clone();
                        let seed_length: usize = line.get(2).unwrap().clone();

                        (seed, seed + seed_length - 1)
                    };

                    println!("Processing map from {} to {} destination: {}", first_source_seed, last_source_seed, first_destination_seed);
    
                    // Slice of the beginning
                    if first_seed < first_source_seed && first_source_seed <= last_seed {   
                        println!("Cutting of the beginning [{}] < ({}) <= [{}]",
                            first_seed, first_source_seed, last_seed
                        );
 
                        final_ranges.push((first_seed, first_source_seed - first_seed + 1));
                        first_seed = first_source_seed;
                    }

                    // Slice of the end
                    if first_seed <= last_source_seed && last_source_seed < last_seed {    
                        println!("Cutting of the end [{}] < ({}) <= [{}]",
                            first_seed, last_source_seed, last_seed
                        );

                        final_ranges.push((first_destination_seed + first_seed - first_source_seed, last_source_seed - first_seed + 1));
                        first_seed = last_source_seed + 1;
                    }

                    if first_source_seed <= first_seed && last_seed <= last_source_seed {
                        println!("Matches range ({}) <= [{}] < [{}] <= ({})",
                            first_source_seed, first_seed, last_seed, last_source_seed
                        );

                        final_ranges.push((
                            first_destination_seed + first_seed - first_source_seed, last_seed - first_seed + 1));    
                        processed = true;                    
                        break;
                    }
                }

                if !processed {
                    println!("Seed [{}], [{}] is outside of range",
                        first_seed, last_seed                
                    );

                    final_ranges.push((first_seed, last_seed - first_seed + 1));
                }
            }    

            println!("Map processed, output ranges: {:?}", final_ranges);
        }

        println!("{:?}", final_ranges.iter().map(|(a, _b)| a).min());

        Ok(())
    }

    use std::{fs, usize};

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc05.txt").expect("Failed to read the input file");
    }

    fn read_aoc_example() -> String {
        return "
        seeds: 79 14

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4
        ".to_string();
    }
}
