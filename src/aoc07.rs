use std::collections::HashSet;

fn read_input(input: &str) -> Vec<(String, usize)> {
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .collect();

    return lines.iter().map(|&line| {
        let parts: Vec<&str> = line.split(' ').map(|part| part.trim()).filter(|part| part.len() > 0).collect();
        let hand: String = parts.get(0).unwrap().to_string();
        let points: usize = parts.get(1).unwrap().parse().unwrap();

        return (hand, points)
    }).collect();
}

fn get_type_rank(hand: &str, use_j_rule: bool) -> usize {
    let number_of_j = hand.matches('J').count();

    {
        // Five of a kind, where all five cards have the same label: AAAAA
        let first_character = hand.chars().nth(0).unwrap();
        if hand.matches(first_character).count() == 5 { return 6; }

        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        let second_character = hand.chars().nth(1).unwrap();
        if hand.matches(first_character).count() == 4 || hand.matches(second_character).count() == 4 { 
            if use_j_rule && (number_of_j == 1 || number_of_j == 4) {
                return 6;
            } else {
                return 5; 
            }
        }
    }

    {
        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        let all_chars: HashSet<char> = hand.chars().collect();
        if all_chars.len() == 2 { 
            if use_j_rule && (number_of_j == 2 || number_of_j == 3) {
                return 6;
            } else {
                return 4; 
            }
         }
    }

    {
        let all_chars: HashSet<char> = hand.chars().collect();
        if all_chars.len() == 3 { 
            let max_occurence = all_chars.iter().map(|&c| hand.matches(c).count()).max().unwrap();

            // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
            if max_occurence == 3 { 
                if use_j_rule && number_of_j == 3 {
                    return 5;
                } else if use_j_rule && number_of_j == 1 {
                    return 5;
                } else {
                    return 3; 
                }
            }

            // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
            if max_occurence == 2 { 
                if use_j_rule && number_of_j == 2 {
                    return 5;
                } else if use_j_rule && number_of_j == 1 {
                    return 4;
                } else {
                    return 2; 
                }
            }
        }

        if all_chars.len() == 4 { 
            if use_j_rule && number_of_j == 2 {
                return 3;
            } else if use_j_rule && number_of_j == 1 {
                return 3;
            } else {
                return 1; 
            }    
        }
    }

    if use_j_rule && number_of_j > 0 {
        return 1;
    } else {
        return 0; 
    }   
}

fn get_card_rank(c: char, use_j_rule: bool) -> usize {
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    return match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if use_j_rule { 1 } else { 11 },
        'T' => 10,
        _ => c.to_string().parse().unwrap(),
    }
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
    fn test_get_type_rank() -> Result<(), String> {
        assert_eq!(get_type_rank("AAAAA", false), 6);
        assert_eq!(get_type_rank("00000", false), 6);
        assert_eq!(get_type_rank("11111", false), 6);
        assert_eq!(get_type_rank("CCCCC", false), 6);

        assert_eq!(get_type_rank("JJJJJ", true), 6);

        assert_eq!(get_type_rank("ABBBB", false), 5);
        assert_eq!(get_type_rank("12111", false), 5);
        assert_eq!(get_type_rank("22232", false), 5);
        assert_eq!(get_type_rank("DDDDC", false), 5);

        assert_eq!(get_type_rank("AJJJJ", true), 6);
        assert_eq!(get_type_rank("1J111", true), 6);

        assert_eq!(get_type_rank("AABBB", false), 4);
        assert_eq!(get_type_rank("21212", false), 4);
        assert_eq!(get_type_rank("DCCCD", false), 4);
        assert_eq!(get_type_rank("CCDDD", false), 4);

        assert_eq!(get_type_rank("JJBBB", true), 6);
        assert_eq!(get_type_rank("DJJJD", true), 6);

        assert_eq!(get_type_rank("ADBBB", false), 3);
        assert_eq!(get_type_rank("20212", false), 3);
        assert_eq!(get_type_rank("DCCCF", false), 3);
        assert_eq!(get_type_rank("CHDDD", false), 3);

        assert_eq!(get_type_rank("AJBBB", true), 5);
        assert_eq!(get_type_rank("J0J1J", true), 5);

        assert_eq!(get_type_rank("ADBAB", false), 2);
        assert_eq!(get_type_rank("20211", false), 2);
        assert_eq!(get_type_rank("DCDCF", false), 2);
        assert_eq!(get_type_rank("CHDHD", false), 2);

        assert_eq!(get_type_rank("AAJBB", true), 4);
        assert_eq!(get_type_rank("220JJ", true), 5);

        assert_eq!(get_type_rank("ADBAX", false), 1);
        assert_eq!(get_type_rank("20213", false), 1);
        assert_eq!(get_type_rank("DCGCF", false), 1);
        assert_eq!(get_type_rank("CGDHD", false), 1);

        assert_eq!(get_type_rank("JDJAX", true), 3);
        assert_eq!(get_type_rank("2J213", true), 3);

        assert_eq!(get_type_rank("12345", false), 0);
        assert_eq!(get_type_rank("ABCDE", false), 0);

        assert_eq!(get_type_rank("123J5", true), 1);

        Ok(())
    }

    #[test]
    fn aoc07_part12() -> Result<(), String> {
        let use_j_rule = true;

        let input = read_aoc_challenge();
        let mut hands = read_input(&input);

        hands.sort_by(|a, b| {
            let hand_a = &a.0;
            let hand_a_rank = get_type_rank(&hand_a, use_j_rule);
            let hand_b = &b.0;
            let hand_b_rank = get_type_rank(&hand_b, use_j_rule);

            if hand_a_rank == hand_b_rank {
                let hand_a_value: Vec<usize> = hand_a.chars()
                    .map(|c| get_card_rank(c,use_j_rule))
                    .collect();

                let hand_b_value: Vec<usize> = hand_b.chars()
                    .map(|c| get_card_rank(c, use_j_rule))
                    .collect();

                let pairs = hand_a_value.into_iter()
                    .zip(hand_b_value.into_iter())
                    .collect::<Vec<(usize, usize)>>();
                
                for (a, b) in pairs {
                    if a != b {
                        return a.cmp(&b);            
                    }
                }

                return Ordering::Equal;
            } else {
                return hand_a_rank.cmp(&hand_b_rank);
            }  
        });

        println!("{:?}", hands);
        
        let solution = hands.iter().enumerate().map(|(i, (_hand, value))| (i + 1) * value).sum::<usize>();

        println!("{}", solution);

        Ok(())
    }

    use std::{fs, cmp::Ordering};

    fn read_aoc_challenge() -> String {
        return fs::read_to_string("src/aoc07.txt").expect("Failed to read the input file");
    }

    fn read_aoc_example() -> String {
        return "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        ".to_string();
    }
}
