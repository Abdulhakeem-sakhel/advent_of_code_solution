use std::collections::HashMap;
use std::fs;
fn main() {
    let file_path = "input.txt";
    let binding = fs::read_to_string(file_path)
        .unwrap();
    let network = binding.as_str();
    
    let instructions = network.lines().next().unwrap();
    let network_map = get_network(network);
    let step = number_of_steps(instructions, network_map);
    println!("the result = {}", step);
}

fn get_network(network: &str) -> HashMap<&str, (&str, &str)> {
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

fn get_node_ends_A<'a>(network_map: &'a HashMap<&'a str, (&'a str, &'a str)>) -> Vec<&'a str> {
    network_map.keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| *node)
        .collect()
}

fn number_of_steps(instructions: &str, network_map: HashMap<&str, (&str, &str)>) -> usize {
    let mut steps_number: usize = 0;
    let instructions_len = instructions.len();
    let mut current_nodes = get_node_ends_A(&network_map);
    loop {
        let current_move: char = instructions.chars().nth(steps_number % instructions_len).unwrap();
        
        for  node in current_nodes.iter_mut() {
            match current_move {
                'R' => {
                    *node =  network_map.get(node).unwrap().1
                },
                'L' => {
                    *node = network_map.get(node).unwrap().0
                },
                ins => panic!("i get an unexpected instructions !!!! ({ins})"),
            };
        }
        steps_number += 1;

        let mut all_ends_Z = true;
        for i in 0..current_nodes.len() {
            if !current_nodes[i].ends_with("Z"){
                all_ends_Z = false;
                break;
            }
        }
        if all_ends_Z == true {
            break;
        }
    }
    println!("{:?}", current_nodes);
    return steps_number;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let network = 
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX))";
        
        let instructions = network.lines().next().unwrap();
        println!("{instructions}");
        let network_map = get_network(network);
        let step = number_of_steps(instructions, network_map);
        //assert_eq!(true, "11B".ends_with("Z"));
        assert_eq!(6, step);
    }
}