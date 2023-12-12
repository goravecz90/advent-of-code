use std::{rc::Rc, collections::HashMap, cell::RefCell};

#[derive(Clone, Debug)]
struct Node {
    value: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}

fn read_input(input: &str, single_start: bool) -> (Vec<Rc<RefCell<Node>>>, Vec<char>) {
    let mut lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line: &&str| line.len() > 0)
        .collect();

    let mut node_map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    let get_node_or_create = |node_map: &mut HashMap<String, Rc<RefCell<Node>>>, node_value: &str| -> Rc<RefCell<Node>> {
        match node_map.get(node_value) {
            Some(node) => node.clone(),
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    value: node_value.to_string(),
                    left: None,
                    right: None,
                }));
    
                node_map.insert(node_value.to_string(), new_node.clone());
                return new_node;
            }
        }
    };

    let steps = lines.remove(0).chars().collect::<Vec<char>>();

    for line in lines {
        let parts: Vec<&str> = line.split('=')
            .map(|line| line.trim())
            .filter(|line: &&str| line.len() > 0)
            .collect();

        let left_and_right: Vec<String> = parts.get(1).unwrap().split(',')
            .map(|part| part.to_owned())
            .map(|part| part.replace('(', ""))
            .map(|part| part.replace(')', ""))
            .map(|part| part.trim().to_owned())
            .collect();

        let left_node_value = left_and_right.get(0).unwrap().trim();
        let left_node = get_node_or_create(&mut node_map, left_node_value);

        let right_node_value = left_and_right.get(1).unwrap().trim();
        let right_node = get_node_or_create(&mut node_map, right_node_value);

        let node_value = parts.get(0).unwrap().trim();
        let node = get_node_or_create(&mut node_map, node_value);

        {
            let mut node_mut = node.borrow_mut();
            node_mut.left = Option::Some(left_node);
            node_mut.right = Option::Some(right_node);
        }
    }

    for (key, value) in &node_map {
        let left_value = match &value.borrow().left {
            Some(node) => node.borrow().value.clone(),
            None => String::from("None"),
        };

        let right_value = match &value.borrow().right {
            Some(node) => node.borrow().value.clone(),
            None => String::from("None"),
        };

        println!("{} = ({}, {})", key, left_value, right_value)
    }

    let starting_nodes: Vec<Rc<RefCell<Node>>> = if single_start {
        let node: Rc<RefCell<Node>> = node_map.iter()
            .filter(|&(key, _value)| key == "AAA")
            .map(|(_key, value)| value.clone())
            .next().unwrap();

        vec![node]
    } else {
        node_map.iter()
            .filter(|&(key, _value)| {
                key.ends_with('A')
            })
            .map(|(_key, value)| value.clone())
            .collect()
    };

    return (starting_nodes, steps);
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::integer::lcm;

    #[test]
    fn test_parsing() -> Result<(), String> {
        let input = read_aoc_example();

        read_input(&input, false);

        Ok(())
    }

    #[test]
    fn aoc08_part1() -> Result<(), String> {
        let single_start = true;

        let mut steps_taken: usize = 0;
        let input = read_aoc_challenge();

        let (start_nodes, steps) = read_input(&input, single_start);
        // print!("Start nodes: ");
        // for node in &start_nodes {
        //     print!("{} ", node.borrow().value);
        // }
        // println!("");

        let mut current_nodes: Vec<Rc<RefCell<Node>>>  = start_nodes.iter()
            .map(|node| node.clone())
            .collect();

        for step in steps.iter().cycle() {
            steps_taken += 1;
            // if steps_taken > 10000000000 { 
            //     println!("Too many steps!");
            //     return Ok(())
            // }
            if steps_taken % 1000000 == 0 {
                println!("Step {}!", steps_taken);
            }
            
            current_nodes = current_nodes.iter().map(|current_node| match step {
                'L' => current_node.borrow().left.as_ref().unwrap().clone(),
                'R' => current_node.borrow().right.as_ref().unwrap().clone(),
                _ => panic!("Oh no!")
            }).collect();

            if current_nodes.len() == 1 {
                if current_nodes.get(0).unwrap().borrow().value == "ZZZ" {
                    break;
                }
            } else {
                let has_all_ended = current_nodes.iter().all(|current_node| 
                    current_node.borrow().value.ends_with("Z")
                );

                if has_all_ended {
                    break;
                }
            }
            
            // print!("Current nodes: ");
            // for current_node in &current_nodes {
            //     print!("{} ", current_node.borrow().value);
            // }
            // println!("");        
        }

        println!("{}", steps_taken);


        Ok(())
    }

    #[test]
    fn aoc08_part2() -> Result<(), String> {
        let single_start = false;
        let input = read_aoc_challenge();

        let (start_nodes, steps) = read_input(&input, single_start);

        let loop_lengths: Vec<u128> = start_nodes.iter()
            .map(|node| {
                let mut current_node = node.clone();
                let mut steps_taken: usize = 0;

                for step in steps.iter().cycle() {
                    steps_taken += 1;

                    current_node = match step {
                        'L' => current_node.borrow().left.as_ref().unwrap().clone(),
                        'R' => current_node.borrow().right.as_ref().unwrap().clone(),
                        _ => panic!("Oh no!")
                    };

                    if current_node.borrow().value.ends_with("Z") {
                        break;
                    }
                }

                steps_taken as u128
            })
            .collect();

        let length = loop_lengths.iter().fold(1u128, |acc, &num| lcm(acc, num));
        println!("{}", length);

        Ok(())
    }

    use std::fs;

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc08.txt").expect("Failed to read the input file");
    }

    fn read_aoc_example() -> String {
        return "
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        ".to_string();
    }

    fn read_aoc_example_2() -> String {
        return "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        ".to_string();
    }
}
