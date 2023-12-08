use std::collections::HashMap;
use std::fs;
fn main() {
    let file_path = "input.txt";
    let binding = fs::read_to_string(file_path)
        .unwrap();
    let network = binding.as_str();
    
    let instructions = network.lines().next().unwrap();
    let network_map = get_netWork(network);
    let step = number_of_steps(instructions, network_map);
    println!("the result = {}", step);
}

fn get_netWork(network: &str) -> HashMap<&str, (&str, &str)> {
    let mut network_map: HashMap<&str, (&str, &str)> = HashMap::new();

    network.lines().enumerate()
        .for_each(|(i, line)| {
            if i  >= 2 {
                let key = line.split("=")
                    .nth(0)
                    .unwrap().trim();
                let left = line.split("=")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .nth(0)
                    .unwrap()
                    .trim_start_matches(" (");
                let right = line.split("=")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .nth(1)
                    .unwrap()
                    .trim_end_matches(")")
                    .trim();
                network_map.insert(key, (left, right));
            }
        });
    return network_map;
}

fn number_of_steps(instructions: &str, network_map: HashMap<&str, (&str, &str)>) -> usize {
    let mut steps_number: usize = 0;
    let instructions_len = instructions.len();
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let current_move: char = instructions.chars().nth(steps_number % instructions_len).unwrap();
        current_node = match current_move {
            'R' => network_map.get(current_node).unwrap().1,
            'L' => network_map.get(current_node).unwrap().0,
            ins => panic!("i get an unexpected instructions !!!! ({ins})"),
        };
        steps_number += 1;
    }

    return steps_number;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let network = 
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        
        let instructions = network.lines().next().unwrap();
        println!("{instructions}");
        let network_map = get_netWork(network);
        let step = number_of_steps(instructions, network_map);
        assert_eq!(6, step);
    }
}